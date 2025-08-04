use crate::background;

use super::*;

use tracing::{debug, warn};

use std::path::{Path, PathBuf};

pub struct FileWalker {
    file_list: Vec<PathBuf>,

    /// Branch to index files as part of.
    branch: String,
}

impl FileWalker {
    pub fn index_directory(dir: impl AsRef<Path>, branch: String) -> impl FileSource {
        // note: this WILL observe .gitignore files for the respective repos.
        let walker = ignore::WalkBuilder::new(&dir)
            .standard_filters(true)
            .hidden(false)
            .build();

        let file_list = walker
            .filter_map(|de| match de {
                Ok(de) => Some(de),
                Err(err) => {
                    warn!(%err, "access failure; skipping");
                    None
                }
            })
            .filter(|de| !de.path().strip_prefix(&dir).unwrap().starts_with(".git"))
            .filter_map(|de| crate::canonicalize(de.into_path()).ok())
            .collect();

        Self { file_list, branch }
    }

    pub fn from_paths(root: &Path, paths: Vec<PathBuf>) -> Self {
        use std::collections::HashSet;

        let mut unique = HashSet::new();
        let mut file_list = vec![];
        for path in paths {
            let full = if path.is_absolute() {
                path
            } else {
                root.join(path)
            };
            if let Ok(canon) = crate::canonicalize(full) {
                if unique.insert(canon.clone()) {
                    file_list.push(canon);
                }
            }
        }

        Self {
            file_list,
            branch: "HEAD".into(),
        }
    }
}

impl FileSource for FileWalker {
    fn len(&self) -> usize {
        self.file_list.len()
    }

    fn for_each(self, pipes: &SyncPipes, iterator: impl Fn(RepoDirEntry) + Sync + Send) {
        use rayon::prelude::*;
        background::rayon_pool().install(|| {
            self.file_list
                .into_par_iter()
                .filter_map(|entry_disk_path| {
                    if entry_disk_path.is_file() {
                        let path = entry_disk_path.clone();
                        let buffer = Box::new(move || match std::fs::read_to_string(&path) {
                            Err(err) => {
                                warn!(%err, entry_disk_path=?path, "read failed; skipping");
                                Err(err)
                            }
                            Ok(buffer) => Ok(buffer),
                        });
                        Some(RepoDirEntry::File(RepoFile {
                            buffer,
                            len: entry_disk_path.metadata().ok()?.len(),
                            path: entry_disk_path.to_string_lossy().to_string(),
                            branches: vec![self.branch.clone()],
                        }))
                    } else if entry_disk_path.is_dir() {
                        Some(RepoDirEntry::Dir(RepoDir {
                            path: entry_disk_path.to_string_lossy().to_string(),
                            branches: vec![self.branch.clone()],
                        }))
                    } else {
                        debug!(?entry_disk_path, "skipping entry, not a file or directory");
                        None
                    }
                })
                .take_any_while(|_| !pipes.is_cancelled())
                .for_each(iterator);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn from_paths_collects_files() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        std::fs::write(&file, "a").unwrap();

        let walker = FileWalker::from_paths(dir.path(), vec![file.clone()]);
        assert_eq!(walker.len(), 1);
    }
}

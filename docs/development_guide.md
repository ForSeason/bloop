# 开发指南

本文档汇总了从源码构建和常见开发任务的步骤。

## 环境准备
1. 安装 `git-lfs`，并在仓库中执行：
   ```bash
   git lfs install
   git lfs pull
   ```
   【F:README.md†L71-L78】
2. 创建 `local_config.json`，填入 GitHub 与 OpenAI 凭证【F:README.md†L46-L52】。

### 构建后端
后端位于 `server/bleep`，需要 `rust` 与 `onnxruntime` 等依赖【F:server/README.md†L8-L12】。构建命令：
```bash
cargo build -p bleep --release
```
【F:server/README.md†L13-L17】

运行并索引本地目录的示例：
```bash
cargo run -p bleep --release -- \
  --source-dir /path/to/dir
```
【F:server/README.md†L21-L26】

### 构建桌面应用
桌面端依赖 `rustup`、`clang`、`cmake` 等工具【F:apps/desktop/README.md†L5-L18】。在根目录执行：
```bash
npm install
npm run build-app
```
或使用开发模式：
```bash
npm run start-app
```
【F:apps/desktop/README.md†L24-L41】

### 运行前端（浏览器模式）
确保后端已运行，并在 `.env` 中设置 `API_URL`，然后执行：
```bash
npm install
npm run start-web
```
打开 `localhost:5173` 即可访问【F:client/README.md†L3-L12】。

### 测试
项目包含基于 Playwright 的端到端测试，可通过以下命令运行：
```bash
npx playwright test
```
测试配置位于 `playwright.config.js`【F:playwright.config.js†L1-L39】。

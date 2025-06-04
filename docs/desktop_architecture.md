# Tauri 桌面应用架构

桌面端位于 `apps/desktop`。其前端代码与 `client` 共用，入口文件 `src/main.tsx` 仅负责在浏览器上下文渲染 `App` 组件【F:apps/desktop/src/main.tsx†L1-L8】。

Tauri 后端位于 `src-tauri`，其中 `backend.rs` 调用 `bleep` 库启动本地搜索服务并向前端暴露命令接口【F:apps/desktop/src-tauri/src/backend.rs†L1-L10】。`main.rs` 负责初始化 Tauri 应用、加载插件并在后台持续运行【F:apps/desktop/src-tauri/src/main.rs†L1-L20】。

桌面应用通过与本地 `bleep` 实例通信提供离线索引与搜索能力，同时可在窗口内嵌入 React 前端界面。

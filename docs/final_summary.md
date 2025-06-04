# 项目概览

bloop 是一个结合 Rust 与 JavaScript/TypeScript 的跨平台代码搜索与 AI 辅助工具。其核心特性、安装方法和贡献指南在 `README.md` 中已给出。

该仓库主要包含三大部分：
1. `server/bleep` —— Rust 实现的后端搜索服务；
2. `client` —— React 编写的前端界面，可在浏览器或桌面端运行；
3. `apps/desktop` —— 基于 Tauri 的桌面应用包装。

更详细的架构说明请参见下列文档：
- [`architecture_overview.md`](architecture_overview.md)
- [`server_architecture.md`](server_architecture.md)
- [`client_architecture.md`](client_architecture.md)
- [`desktop_architecture.md`](desktop_architecture.md)

新开发者可阅读 `development_guide.md` 了解环境搭建与常用脚本。

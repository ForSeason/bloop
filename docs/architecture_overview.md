# 架构概览

bloop 由三个主要部分组成：

1. **server/bleep** – Rust 实现的后端服务，负责仓库索引、代码检索和语义搜索。
2. **client** – 使用 React 构建的前端，既可在浏览器运行，也可与桌面端共享界面代码。
3. **apps/desktop** – 基于 Tauri 的桌面应用，封装前端并与本地后端服务交互。

各组件之间通过 HTTP API 进行通信，桌面端与浏览器端都以 `client` 产生的 UI 为核心，与后端的 `bleep` 交互完成索引与查询。

下面的文档将分别介绍 server、client 与 desktop 的内部结构。

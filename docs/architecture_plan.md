# 架构文档规划

根据现有代码与文档，可将 bloop 的架构拆分为以下几部分进行描述：

1. **整体组件概览**
   - 项目目录结构与各组件职责。
   - 三大子项目（server、client、desktop）的关系与交互方式。

2. **后端 `bleep` 服务架构**
   - Application 初始化流程及核心结构。
   - 索引系统：Tantivy 文档索引与 Qdrant 语义索引的构建与更新流程。
   - Web API 设计：主要路由及其功能，如搜索、项目管理等。
   - 与 GitHub、本地仓库的同步机制。

3. **前端 React 客户端结构**
   - 主要页面与组件目录。
   - 状态管理与上下文（context）使用方式。
   - 与后台 API 的交互方式。

4. **Tauri 桌面应用**
   - 基于 Tauri 的项目结构以及启动流程。
   - 与前端共享的代码及特定的桌面功能（如本地缓存管理）。

以上各部分将分别撰写独立文档，以便后续扩展。计划生成的文档如下：
- `server_architecture.md`
- `client_architecture.md`
- `desktop_architecture.md`
- `architecture_overview.md`（汇总整体视角）

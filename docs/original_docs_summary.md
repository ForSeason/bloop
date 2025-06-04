# 项目文档总结

本仓库包含一个跨平台的代码检索与 AI 助手项目 *bloop*。根目录的 `README.md` 介绍了项目功能、快速开始以及贡献指南等内容。

## 项目特点
- 自然语言对话式搜索与代码导航
- 支持基于代码的 LLM playground —— Code Studio
- 正则搜索与符号搜索
- 支持超过十种语言的语法分析，依赖 [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- 采用 [Tantivy](https://github.com/quickwit-oss/tantivy) 与 [Qdrant](https://github.com/qdrant/qdrant) 构建搜索索引，并通过 [Tauri](https://github.com/tauri-apps/tauri) 提供桌面应用

## 快速开始
- 最简单的方式是直接下载发行版，或参考 [Build bloop app from source](../apps/desktop/README.md) 与 [Run bloop from the command line](../server/README.md) 完成本地构建【F:README.md†L33-L40】。
- 如需从源码运行，需要创建 `local_config.json` 提供 GitHub 与 OpenAI 的访问密钥【F:README.md†L46-L55】。

## 子项目说明
- `apps/desktop`：基于 Tauri 的桌面端应用【F:README.md†L67-L69】。
- `server/bleep`：Rust 编写的核心搜索与导航后端【F:README.md†L67-L69】。
- `client`：React 实现的前端界面【F:README.md†L67-L69】。

### Server 文档
`server/README.md` 介绍了 `bleep` 包的安装与使用方式，包括如何索引本地目录、同步 GitHub 仓库以及查询示例【F:server/README.md†L21-L49】。

### Desktop 文档
`apps/desktop/README.md` 详细列出了构建桌面应用所需的依赖与基本命令，例如 `npm run build-app`、`npm run start-app` 等【F:apps/desktop/README.md†L24-L41】。

### Client 文档
`client/README.md` 描述了在浏览器中运行前端的方法，需要先启动后端服务并设置 `API_URL`，然后执行 `npm run start-web`【F:client/README.md†L3-L12】。


# `bleep` 后端架构

`server/bleep` 是项目的核心后端服务，采用 Rust 编写，负责仓库索引、搜索与语义分析等功能。

## 主要模块
源码入口位于 `src/lib.rs`，其中定义了 `Application` 结构体作为全局状态管理中心，包括配置、索引、数据库及后台任务队列等字段【F:server/bleep/src/lib.rs†L82-L111】。

初始化流程在 `Application::initialize` 中完成：
1. 解析配置并设置线程数、缓存大小等参数。
2. 初始化仓库池、SQL 数据库以及语义搜索引擎 `Semantic`。
3. 如检测到索引版本变更，会重建 Tantivy 与语义索引并清理缓存【F:server/bleep/src/lib.rs†L120-L155】。
4. 创建 `Indexes` 实例，用于管理 repo、file 与文档索引【F:server/bleep/src/lib.rs†L156-L163】。

运行服务时，`Application::run` 会根据配置决定是否仅构建索引或同时启动 Web 服务器，并在后台执行同步与定时任务【F:server/bleep/src/lib.rs†L198-L224】。

## 索引系统
`Indexes` 结构体包装了三类索引：仓库索引、文件索引以及文档索引。其构建过程会根据配置在磁盘创建或打开相应数据库【F:server/bleep/src/indexes.rs†L79-L113】。当检测到 schema 变更时，`reset_databases` 会删除旧索引以保证兼容【F:server/bleep/src/indexes.rs†L117-L128】。

索引写入通过 `writers()` 方法获得写句柄，内部使用互斥锁保证同一时间只有一个写入任务【F:server/bleep/src/indexes.rs†L131-L140】。

## 查询解析
查询语言由 `query` 模块处理。`Query` 结构体描述了搜索参数，如仓库、路径、语言及目标类型【F:server/bleep/src/query/parser.rs†L7-L24】。解析后的查询会被用于构建 Tantivy 查询或触发语义搜索。

## Web 服务
`webserver` 模块基于 `axum` 提供 HTTP API，包含搜索、项目管理等路由。服务启动入口位于 `webserver::start`，具体路由在 `webserver` 各子模块中实现，例如 `search.rs` 处理搜索请求。

## 同步与后台任务
`background` 和 `periodic` 模块负责定时扫描仓库变更、同步 GitHub 以及日志轮转等维护任务。后台任务运行在独立的线程池中，由 `SyncQueue` 管理。

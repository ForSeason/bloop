# React 前端架构

`client` 目录包含浏览器和桌面端共用的 React 应用。核心入口文件为 `App.tsx`，其中组合了项目、命令栏、设置等页面，并通过多种 context 管理状态【F:client/src/App.tsx†L1-L18】。

## 目录结构
- `components/`：通用 UI 组件，如按钮、下拉框等。
- `CommandBar/`：命令面板及教学步骤。
- `Project/`、`Settings/` 等：对应主要页面逻辑。
- `context/`：React context 定义与 Provider，例如 `projectContext.ts` 用于管理当前项目状态【F:client/src/context/projectContext.ts†L1-L20】。

## 与后端交互
前端通过 Axios 等库向 `bleep` 提供的 HTTP API 发送请求。开发或浏览器模式下，可在根目录运行 `npm run start-web` 启动本地服务器，并通过设置 `API_URL` 指定后端地址【F:client/README.md†L3-L12】。

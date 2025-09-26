# Redis View

基于 Tauri + Vue + TypeScript 的 Redis 可视化管理工具。

## 特性

- 基于 Vue 3 Composition API + TypeScript
- 使用 Tauri 构建跨平台桌面应用
- 支持 Redis 数据可视化管理
- 支持 Redis 常用数据类型操作
- 支持连接多个 Redis 实例

## 开发环境搭建

### 推荐开发环境

- [VS Code](https://code.visualstudio.com/)
- [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (Vue 语言支持)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) (Tauri 开发支持)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) (Rust 语言支持)

### 启动开发环境

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev
```

## 构建应用

```bash
# 构建生产环境版本
pnpm build

# 构建并打包桌面应用
pnpm build:tauri
```

## 项目结构

```
├── src-tauri/         # Tauri 后端代码
├── src/               # Vue 前端代码
├── package.json       # 项目依赖配置
├── tsconfig.json      # TypeScript 配置
└── vite.config.ts     # Vite 构建配置
```

## 技术栈

- 前端：Vue 3 + TypeScript + Vite
- 后端：Rust + Tauri
- 构建工具：Vite + Pnpm
- 跨平台：支持 Windows、macOS、Linux

## 贡献指南

欢迎贡献代码和改进项目。请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 获取贡献指南。

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。
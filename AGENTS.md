# AGENTS.md

## 单仓管理规范

- 本仓是多语言工具箱的单仓（monorepo），每种语言一个单仓，位于 `packages/{lang}/`
- 每个单仓是一个独立 workspace，不跨语言依赖
- 当前已有：`packages/rust/`（Rust 弹舱）

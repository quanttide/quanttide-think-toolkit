# AGENTS.md

## 单仓管理规范

- 本仓是多语言工具箱的单仓（monorepo），每种语言一个弹舱，位于 `packages/{lang}/`
- 每个弹舱是一个独立 workspace，不跨语言依赖
- 当前已有：`packages/rust/`（Rust 弹舱）

## Rust 弹舱规范

- Workspace 根包名 `quanttide-think`，四个 crate 互相独立、无编译期依赖
- 修改 crate 后：先提交推送本仓，再更新主仓库的子模块指针
- Cargo.lock 必须跟踪（workspace 非 library）

## 包管理

- 添加 crate：`cargo new crates/{name}`，再编辑 `Cargo.toml` 加入 `members`
- crate 命名前缀 `qtcloud-think-`，crates.io 发布时用全名

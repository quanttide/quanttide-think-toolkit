# AGENTS.md

## Rust项目规范

- Workspace 根包名 `quanttide-think`，四个 crate 互相独立、无编译期依赖
- 修改 crate 后：先提交推送本仓，再更新主仓库的子模块指针
- Cargo.lock 必须跟踪（workspace 非 library）

### 包管理

- 添加 crate：`cargo new crates/{name}`，再编辑 `Cargo.toml` 加入 `members`
- crate 命名前缀 `qtcloud-think-`，crates.io 发布时用全名

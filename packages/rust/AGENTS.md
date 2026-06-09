# AGENTS.md

## Rust项目规范

- Workspace 根包名 `quanttide-think`，四个 crate 互相独立、无编译期依赖
- 修改 crate 后：先提交推送本仓，再更新主仓库的子模块指针
- Cargo.lock 必须跟踪（workspace 非 library）

### 包管理

- 添加 crate：`cargo new crates/{name}`，再编辑 `Cargo.toml` 加入 `members`
- 目录名使用简短名称（如 `thought`、`intention`），Cargo.toml 中 `name` 使用全名（`quanttide-think-thought`），crates.io 发布时用全名

# AGENTS.md

## Rust项目规范

- 单包 `quanttide-think`，所有类型在 `src/` 下按模块组织
- 修改后：先提交推送本仓，再更新主仓库的子模块指针
- Cargo.lock 必须跟踪（二进制包）

### 模块管理

- 每个数据类型一个模块文件：`src/{thought,intention,situation,schema}.rs`
- `src/lib.rs` 声明 `pub mod` 并 `pub use` 重导出全部公共类型
- 集成测试放在 `tests/` 目录，按模块命名（`thought.rs`、`intention.rs` 等）

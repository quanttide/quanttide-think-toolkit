# quanttide-think

量潮思考云 Rust 工具链。

## 模块

| 模块 | 类型 | 说明 |
|------|------|------|
| `thought` | 流体域 | 想法：id, title, description, created_at |
| `intention` | 固体域 | 意图：title, description, motivation, agent, level, priority, trigger, risk |
| `situation` | 固体域 | 情境：agenda, ecology, frame, dynamics |
| `schema` | 固体域 | 图式：entities, causals, boundaries, properties, dynamics, mappings, biases |

所有类型通过根包 `quanttide-think` 统一导出：

```rust
use quanttide_think::{Thought, Intention, Situation, Schema};
use quanttide_think::schema::{Entity, Causal, Mapping};
```

## 构建

```bash
cargo build
cargo test
```

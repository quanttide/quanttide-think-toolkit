# quanttide-think

量潮认知工程 Rust 工具箱。定义四种认知构件的数据模型：想法、意图、情境、图式。

## 模块

| 模块 | 说明 |
|------|------|
| `thought` | 想法：id, title, description, created_at |
| `intention` | 意图：title, description, motivation, agent, level, priority, trigger, risk |
| `situation` | 情境：agenda, ecology, frame, dynamics |
| `schema` | 图式：entities, causals, boundaries, properties, dynamics, mappings, biases |

## 用法

```rust
use quanttide_think::{Thought, Intention, Situation, Schema};
use quanttide_think::schema::{Entity, Causal, Mapping};
```

## 构建

```bash
cargo build
cargo test
```

## 许可

Apache 2.0

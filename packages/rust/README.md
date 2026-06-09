# quanttide-think

量潮思考云 Rust 工具链。

## Crates

| Crate | 类型 | 说明 |
|-------|------|------|
| `quanttide-think-thought` | 流体域 | 想法数据类型：id, title, description, created_at |
| `quanttide-think-intention` | 固体域 | 意图数据类型：title, description, motivation, agent, level, priority, trigger, risk |
| `quanttide-think-situation` | 固体域 | 情境数据类型：agenda, ecology, frame, dynamics |
| `quanttide-think-schema` | 固体域 | 图式数据类型：entities, causals, boundaries, properties, mappings, biases |

## 构建

```bash
cargo build
cargo test
```

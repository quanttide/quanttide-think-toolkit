use qtcloud_think_schema::{
    Bias, Causal, Entity, KeyValue, Mapping, Schema, SchemaContent,
};
use serde_json::Value;
use uuid::Uuid;

fn sample_schema() -> Schema {
    Schema {
        id: Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef1234567890").unwrap(),
        name: "business".into(),
        label: "商务拓展".into(),
        content: SchemaContent {
            usage: "适用于产品经理和创业者评估新想法时参考".into(),
            entities: vec![Entity {
                name: "新想法".into(),
                attributes: vec!["可行性".into(), "门槛".into()],
            }],
            causals: vec![Causal {
                condition: "新想法通过低门槛验证".into(),
                outcome: "可行后结构化复制".into(),
            }],
            boundaries: vec!["适用于商业和研发探索".into()],
            properties: vec![KeyValue {
                key: "验证成本".into(),
                value: "低".into(),
            }],
            dynamics: vec![KeyValue {
                key: "演化方向".into(),
                value: "从单点到系统".into(),
            }],
            mappings: vec![Mapping {
                intent: "探索新商业模式".into(),
                action: Value::String("做POC验证".into()),
            }],
            biases: vec![Bias {
                id: Uuid::parse_str("b1c2d3e4-f5a6-7890-bcde-f12345678901").unwrap(),
                belief: "所有想法都值得先投入".into(),
                fact: "低门槛验证可快速筛选无效想法".into(),
            }],
        },
    }
}

#[test]
fn construct() {
    let s = sample_schema();
    assert_eq!(s.name, "business");
    assert_eq!(s.content.entities[0].name, "新想法");
}

#[test]
fn yaml_roundtrip() {
    let s = sample_schema();
    let yaml = serde_yaml::to_string(&s).unwrap();
    let back: Schema = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(s, back);
}

#[test]
fn deserialize_gallery_format() {
    let yaml = r#"
id: a1b2c3d4-e5f6-7890-abcd-ef1234567890
name: business
label: 商务拓展
content:
  usage: 适用于产品经理和创业者评估新想法时参考
  entities:
    - name: 新想法
      attributes:
        - 可行性
        - 门槛
  causals:
    - condition: 新想法通过低门槛验证
      outcome: 可行后结构化复制
  boundaries:
    - 适用于商业和研发探索
  properties:
    - key: 验证成本
      value: 低
  dynamics:
    - key: 演化方向
      value: 从单点到系统
  mappings:
    - intent: 探索新商业模式
      action: 做POC验证
  biases:
    - id: b1c2d3e4-f5a6-7890-bcde-f12345678901
      belief: 所有想法都值得先投入
      fact: 低门槛验证可快速筛选无效想法
"#;
    let s: Schema = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(s.name, "business");
    assert_eq!(s.content.entities.len(), 1);
    assert_eq!(s.content.mappings[0].intent, "探索新商业模式");
    assert_eq!(s.content.biases[0].belief, "所有想法都值得先投入");
}

#[test]
fn mapping_action_variants() {
    let yaml_str = r#"
intent: test
action: 做POC验证
"#;
    let m: Mapping = serde_yaml::from_str(yaml_str).unwrap();
    assert_eq!(m.action, Value::String("做POC验证".into()));

    let yaml_number = r#"
intent: test
action: 42
"#;
    let m: Mapping = serde_yaml::from_str(yaml_number).unwrap();
    assert_eq!(m.action, Value::Number(42.into()));
}

use quanttide_think_situation::{Situation, SituationContent};
use uuid::Uuid;

fn sample_situation() -> Situation {
    Situation {
        id: Uuid::parse_str("d4e5f6a7-b8c9-0123-defa-234567890123").unwrap(),
        name: "think".into(),
        label: "认知工程".into(),
        content: SituationContent {
            agenda: "将意图工程确立为认知工程的核心方法论".into(),
            ecology: "人机辩论模型暴露出 AI 自收敛不跳出框架的问题".into(),
            frame: "将人机辩论理解为意图发现和收敛的具体手段".into(),
            dynamics: "从人机辩论与反思模型的实验到可拓展监督模式的确立".into(),
        },
    }
}

#[test]
fn construct() {
    let s = sample_situation();
    assert_eq!(s.name, "think");
    assert_eq!(s.content.agenda, "将意图工程确立为认知工程的核心方法论");
}

#[test]
fn yaml_roundtrip() {
    let s = sample_situation();
    let yaml = serde_yaml::to_string(&s).unwrap();
    let back: Situation = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(s, back);
}

#[test]
fn deserialize_gallery_format() {
    let yaml = r#"
id: d4e5f6a7-b8c9-0123-defa-234567890123
name: think
label: 认知工程
content:
  agenda: 将意图工程确立为认知工程的核心方法论
  ecology: 人机辩论模型暴露出 AI 自收敛不跳出框架的问题
  frame: 将人机辩论理解为意图发现和收敛的具体手段
  dynamics: 从人机辩论与反思模型的实验到可拓展监督模式的确立
"#;
    let s: Situation = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(s.name, "think");
    assert_eq!(s.content.ecology, "人机辩论模型暴露出 AI 自收敛不跳出框架的问题");
}

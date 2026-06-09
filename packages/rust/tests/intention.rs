use quanttide_think::intention::{Intention, Agent, Level, Priority, Trigger, Risk};
use uuid::Uuid;

fn sample_intention() -> Intention {
    Intention {
        id: Uuid::parse_str("2ee4bf46-9b9c-4a34-979d-f88c84ebb714").unwrap(),
        title: "建立人机协同的反思与辩论模型".into(),
        description: "通过 AI 自我辩论和人类反馈的交替收敛实现高质量反思".into(),
        motivation: "发现 AI 自收敛会困在局部最优，人类反馈能跳出框架".into(),
        agent: Agent { name: "founder_ai".into(), label: "创始人+AI".into() },
        level: Level {
            name: "middle".into(),
            label: "中层".into(),
            description: "为顶层意图服务的阶段性目标".into(),
        },
        priority: Priority {
            name: "high".into(),
            label: "高".into(),
            description: "可拓展监督模式被验证有效，推动认知方法论落地".into(),
        },
        trigger: Trigger {
            name: "persistent".into(),
            label: "持续".into(),
            description: "直到辩论模型可稳定产出章程".into(),
        },
        risk: Risk {
            name: "medium".into(),
            label: "中".into(),
            description: "人类介入的时机和频率仍需校准".into(),
        },
    }
}

#[test]
fn construct() {
    let i = sample_intention();
    assert_eq!(i.title, "建立人机协同的反思与辩论模型");
    assert_eq!(i.agent.name, "founder_ai");
}

#[test]
fn yaml_roundtrip() {
    let i = sample_intention();
    let yaml = serde_yaml::to_string(&i).unwrap();
    let back: Intention = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(i, back);
}

#[test]
fn deserialize_gallery_format() {
    let yaml = r#"
id: 2ee4bf46-9b9c-4a34-979d-f88c84ebb714
title: 建立人机协同的反思与辩论模型
description: 通过 AI 自我辩论和人类反馈的交替收敛实现高质量反思
motivation: 发现 AI 自收敛会困在局部最优，人类反馈能跳出框架
agent:
  name: founder_ai
  label: 创始人+AI
level:
  name: middle
  label: 中层
  description: 为顶层意图服务的阶段性目标
priority:
  name: high
  label: 高
  description: 可拓展监督模式被验证有效，推动认知方法论落地
trigger:
  name: persistent
  label: 持续
  description: 直到辩论模型可稳定产出章程
risk:
  name: medium
  label: 中
  description: 人类介入的时机和频率仍需校准
"#;
    let i: Intention = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(i.id.to_string(), "2ee4bf46-9b9c-4a34-979d-f88c84ebb714");
    assert_eq!(i.agent.name, "founder_ai");
    assert_eq!(i.level.name, "middle");
    assert_eq!(i.priority.name, "high");
    assert_eq!(i.trigger.name, "persistent");
    assert_eq!(i.risk.name, "medium");
}

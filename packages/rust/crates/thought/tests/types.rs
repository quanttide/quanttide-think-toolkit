use qtcloud_think_thought::Thought;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[test]
fn construct() {
    let t = Thought {
        id: Uuid::new_v4(),
        title: "test".into(),
        description: "a thought".into(),
        created_at: Utc::now(),
    };
    assert_eq!(t.title, "test");
}

#[test]
fn yaml_roundtrip() {
    let t = Thought {
        id: Uuid::parse_str("d4e5f6a7-b8c9-0123-defa-234567890123").unwrap(),
        title: "测试".into(),
        description: "一段想法".into(),
        created_at: DateTime::parse_from_rfc3339("2026-06-09T10:30:00Z")
            .unwrap()
            .into(),
    };
    let yaml = serde_yaml::to_string(&t).unwrap();
    let back: Thought = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(t, back);
}

#[test]
fn deserialize_gallery_format() {
    let yaml = r#"
id: d4e5f6a7-b8c9-0123-defa-234567890123
title: 测试
description: 一段想法
created_at: 2026-06-09T10:30:00Z
"#;
    let t: Thought = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(t.title, "测试");
    assert_eq!(t.created_at.to_rfc3339(), "2026-06-09T10:30:00+00:00");
}

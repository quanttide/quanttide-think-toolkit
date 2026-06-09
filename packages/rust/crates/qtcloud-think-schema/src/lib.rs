//! Schema — 图式数据类型（固体域）

pub struct Schema {
    pub id: String,
    pub name: String,
    pub label: String,
    pub content: SchemaContent,
}

pub struct SchemaContent {
    pub usage: String,
    pub entities: Vec<serde_yaml::Value>,
    pub causals: Vec<Causal>,
    pub boundaries: Vec<String>,
    pub properties: Vec<KeyValue>,
    pub mappings: Vec<Mapping>,
    pub biases: Vec<Bias>,
}

pub struct Causal {
    pub condition: String,
    pub outcome: String,
}

pub struct KeyValue {
    pub key: String,
    pub value: String,
}

pub struct Mapping {
    pub intent: String,
    pub action: serde_yaml::Value,
}

pub struct Bias {
    pub id: String,
    pub belief: String,
    pub fact: String,
}

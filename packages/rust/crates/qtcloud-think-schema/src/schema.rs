use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub id: Uuid,
    pub name: String,
    pub label: String,
    pub content: SchemaContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaContent {
    pub usage: String,
    pub entities: Vec<Entity>,
    pub causals: Vec<Causal>,
    pub boundaries: Vec<String>,
    pub properties: Vec<KeyValue>,
    pub dynamics: Vec<KeyValue>,
    pub mappings: Vec<Mapping>,
    pub biases: Vec<Bias>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Causal {
    pub condition: String,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    pub intent: String,
    pub action: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bias {
    pub id: Uuid,
    pub belief: String,
    pub fact: String,
}

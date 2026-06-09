use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Situation {
    pub id: Uuid,
    pub name: String,
    pub label: String,
    pub content: SituationContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SituationContent {
    pub agenda: String,
    pub ecology: String,
    pub frame: String,
    pub dynamics: String,
}

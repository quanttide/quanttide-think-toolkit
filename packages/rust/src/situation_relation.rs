use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SituationRelation {
    pub source: String,
    pub target: String,
    pub relation_type: SituationRelationType,
    pub confidence: Confidence,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SituationRelationType {
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "trigger")]
    Trigger,
    #[serde(rename = "evolve")]
    Evolve,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Confidence {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}

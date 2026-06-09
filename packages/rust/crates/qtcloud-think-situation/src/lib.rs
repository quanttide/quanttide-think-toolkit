//! Situation — 情境数据类型（固体域）

pub struct Situation {
    pub id: String,
    pub name: String,
    pub label: String,
    pub content: SituationContent,
}

pub struct SituationContent {
    pub agenda: String,
    pub ecology: String,
    pub frame: String,
    pub dynamics: String,
}

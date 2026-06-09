//! Thought — 想法数据类型（流体域）

pub struct Thought {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

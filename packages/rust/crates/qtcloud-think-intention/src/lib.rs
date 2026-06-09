//! Intention — 意图数据类型（固体域）

pub struct Intention {
    pub id: String,
    pub title: String,
    pub description: String,
    pub motivation: String,
    pub agent: Agent,
    pub level: Level,
    pub priority: Priority,
    pub trigger: Trigger,
    pub risk: Risk,
}

pub struct Agent {
    pub name: String,
    pub label: String,
}

pub struct Level {
    pub name: String,
    pub label: String,
    pub description: String,
}

pub struct Priority {
    pub name: String,
    pub label: String,
    pub description: String,
}

pub struct Trigger {
    pub name: String,
    pub label: String,
    pub description: String,
}

pub struct Risk {
    pub name: String,
    pub label: String,
    pub description: String,
}

//! # quanttide-think
//!
//! 量潮认知工程Rust工具箱。

pub mod thought;
pub mod intention;
pub mod situation;
pub mod schema;
pub mod domain;
pub mod situation_relation;

pub use thought::*;
pub use intention::*;
pub use situation::*;
pub use schema::*;
pub use domain::Domain;
pub use situation_relation::{SituationRelation, RelationType, Confidence};

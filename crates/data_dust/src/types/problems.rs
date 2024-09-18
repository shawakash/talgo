use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::problems)]
pub struct NewProblem {
    pub name: String,
    pub difficulty: String,
    pub statement: String,
    pub input_specification: String,
    pub output_specification: String,
    pub time_limit_ms: i32,
    pub memory_limit_kb: i32,
    pub sample_input: String,
    pub sample_output: String,
    pub notes: Option<String>,
    pub author_id: Uuid,
    pub contest_id: i32,
    pub is_public: bool,
    pub points: Option<i32>,
}

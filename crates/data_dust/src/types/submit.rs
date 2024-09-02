use diesel::{prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::submissions)]
pub struct NewSubmission {
    pub user_id: Uuid,
    pub problem_id: i32,
    pub language: String,
    pub code: String,
    pub contest_id: i32
}

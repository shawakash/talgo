use chrono::NaiveDateTime;

use diesel::{prelude::*, query_builder::QueryId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::sub::SubmissionVerdict;
use crate::enums::user::UserRanks;
use crate::schema::{contests, problems, submissions, users};

#[derive(Queryable, Insertable, Serialize, QueryId, Debug, Selectable, Deserialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub country: Option<String>,
    pub organization: Option<String>,
    pub rating: i32,
    pub max_rating: i32,
    pub rank: UserRanks,
    pub contribution: i32,
    pub friend_count: i32,
    pub is_admin: bool,
    pub is_banned: bool,
    pub last_online: NaiveDateTime,
    pub registration_time: NaiveDateTime,
    pub avatar_url: Option<String>,
    pub github_username: Option<String>,
    pub preferred_language: Option<String>,
    pub problems_solved: i32,
    pub contests_participated: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Serialize, QueryId, Debug, Selectable, Deserialize, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = problems)]
pub struct Problems {
    pub id: i32,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_public: bool,
    pub points: Option<i32>,
    pub solved_count: i32,
    pub attempted_count: i32,
}

#[derive(Queryable, Insertable, Serialize, QueryId, Debug, Selectable, Deserialize, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = submissions)]
pub struct Submissions {
    pub id: i64,
    pub user_id: Uuid,
    pub problem_id: i32,
    pub language: String,
    pub code: String,
    pub status: i16,
    pub execution_time_ms: i32,
    pub memory_used_kb: i32,
    pub submitted_at: NaiveDateTime,
    pub verdict: Option<SubmissionVerdict>,
    pub score: Option<f32>,
    pub test_cases_passed: Option<i32>,
    pub total_test_cases: Option<i32>,
    pub contest_id: i32,
}

#[derive(Queryable, Insertable, Serialize, QueryId, Debug, Selectable, Deserialize, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = contests)]
pub struct Contest {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub duration_seconds: i32,
    pub visibility: String,
    pub contest_type: String,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_rated: bool,
    pub max_participants: Option<i32>,
    pub registration_open: bool,
    pub registration_deadline: Option<NaiveDateTime>,
    pub scoring_system: String,
    pub penalty_seconds: i32,
    pub frozen_time_seconds: Option<i32>,
}

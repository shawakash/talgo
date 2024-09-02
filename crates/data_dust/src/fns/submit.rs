extern crate diesel;
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::types::submit::NewSubmission;
use crate::models::Submissions;

use super::DbError;

pub fn insert_submit(
    con: &mut PgConnection,
    new_submission: NewSubmission,
) -> Result<i64, DbError> {
    use crate::schema::submissions::dsl::*;

    diesel::insert_into(submissions)
        .values(&new_submission)
        .returning(id)
        .get_result::<i64>(con)
        // .map(|inserted_id| inserted_id.to_string())
        .map_err(|e| e.into())
}

pub fn get_submit(
    con: &mut PgConnection,
    id:  i64,
) -> Result<Submissions, DbError> {
    use crate::schema::submissions::dsl::*;

    submissions
        .filter(id.eq(id))
        .select(Submissions::as_select())
        .first(con)
        .map_err(|e| e.into())
}

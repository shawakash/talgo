extern crate diesel;
use crate::enums::sub::SubmissionVerdict;
use crate::models::Submissions;
use crate::types::submit::NewSubmission;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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

pub fn get_submit(con: &mut PgConnection, _id: i64) -> Result<Submissions, DbError> {
    use crate::schema::submissions::dsl::*;

    submissions
        .filter(id.eq(id))
        .select(Submissions::as_select())
        .first(con)
        .map_err(|e| e.into())
}

pub fn get_last_n_pending_submissions(
    con: &mut PgConnection,
    n: i64,
) -> Result<Vec<Submissions>, DbError> {
    use crate::schema::submissions::dsl::*;

    submissions
        .filter(verdict.eq(Some(SubmissionVerdict::Pending)))
        .order(submitted_at.asc())
        .limit(n.into())
        .select(Submissions::as_select())
        .load(con)
        .map_err(|e| e.into())
}

pub fn update_submission_verdict(
    con: &mut PgConnection,
    submission_id: i64,
    new_verdict: SubmissionVerdict,
) -> Result<(), DbError> {
    use crate::schema::submissions::dsl::*;

    diesel::update(submissions.find(submission_id))
        .set(verdict.eq(Some(new_verdict)))
        .execute(con)
        .map(|_| ())
        .map_err(|e| e.into())
}

pub fn update_multiple_submission_verdicts(
    con: &mut PgConnection,
    verdict_updates: Vec<(i64, SubmissionVerdict)>,
) -> Result<(), DbError> {
    use crate::schema::submissions::dsl::*;

    con.transaction::<_, diesel::result::Error, _>(|conn| {
        for (sub_id, new_verdict) in verdict_updates {
            diesel::update(submissions.find(sub_id))
                .set(verdict.eq(Some(new_verdict)))
                .execute(conn)?;
        }
        Ok(())
    })
    .map_err(|e| e.into())
}

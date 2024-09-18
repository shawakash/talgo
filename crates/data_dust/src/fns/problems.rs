use crate::models::Problems;
use crate::types::problems::NewProblem;
use diesel::prelude::*;
use diesel::PgConnection;

use super::DbError;

pub fn insert_problem(con: &mut PgConnection, new_problem: NewProblem) -> Result<i32, DbError> {
    use crate::schema::problems::dsl::*;

    diesel::insert_into(problems)
        .values(&new_problem)
        .returning(id)
        .get_result::<i32>(con)
        .map_err(|e| e.into())
}

pub fn get_problem(con: &mut PgConnection, problem_id: i32) -> Result<Problems, DbError> {
    use crate::schema::problems::dsl::*;

    problems
        .filter(id.eq(problem_id))
        .select(Problems::as_select())
        .first(con)
        .map_err(|e| e.into())
}

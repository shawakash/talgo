use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(
    Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow, diesel::sql_types::SqlType,
)]
#[diesel(sql_type = diesel::sql_types::Varchar)]
pub enum SubmissionVerdict {
    Pending,
    InQueue,
    Processing,
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    CompilationError,
    SystemError,
}

impl ToSql<diesel::sql_types::Text, Pg> for SubmissionVerdict {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SubmissionVerdict::Pending => out.write_all(b"Pending")?,
            SubmissionVerdict::InQueue => out.write_all(b"InQueue")?,
            SubmissionVerdict::Processing => out.write_all(b"Processing")?,
            SubmissionVerdict::Accepted => out.write_all(b"Accepted")?,
            SubmissionVerdict::WrongAnswer => out.write_all(b"WrongAnswer")?,
            SubmissionVerdict::TimeLimitExceeded => out.write_all(b"TimeLimitExceeded")?,
            SubmissionVerdict::MemoryLimitExceeded => out.write_all(b"MemoryLimitExceeded")?,
            SubmissionVerdict::RuntimeError => out.write_all(b"RuntimeError")?,
            SubmissionVerdict::CompilationError => out.write_all(b"CompilationError")?,
            SubmissionVerdict::SystemError => out.write_all(b"SystemError")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<diesel::sql_types::Text, Pg> for SubmissionVerdict {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Pending" => Ok(SubmissionVerdict::Pending),
            b"InQueue" => Ok(SubmissionVerdict::InQueue),
            b"Processing" => Ok(SubmissionVerdict::Processing),
            b"Accepted" => Ok(SubmissionVerdict::Accepted),
            b"WrongAnswer" => Ok(SubmissionVerdict::WrongAnswer),
            b"TimeLimitExceeded" => Ok(SubmissionVerdict::TimeLimitExceeded),
            b"MemoryLimitExceeded" => Ok(SubmissionVerdict::MemoryLimitExceeded),
            b"RuntimeError" => Ok(SubmissionVerdict::RuntimeError),
            b"CompilationError" => Ok(SubmissionVerdict::CompilationError),
            b"SystemError" => Ok(SubmissionVerdict::SystemError),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

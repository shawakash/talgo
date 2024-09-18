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
pub enum UserRanks {
    Newbie,
    Pupil,
    Speacialist,
    Expert,
    CandidateMaster,
    Master,
    InternationalMaster,
    Grandmaster,
    InternationalGrandmaster,
    LegendaryGrandmaster,
}

impl ToSql<diesel::sql_types::Text, Pg> for UserRanks {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserRanks::Pupil => out.write_all(b"Pupil")?,
            UserRanks::Speacialist => out.write_all(b"Speacialist")?,
            UserRanks::Expert => out.write_all(b"Expert")?,
            UserRanks::CandidateMaster => out.write_all(b"CandidateMaster")?,
            UserRanks::Master => out.write_all(b"Master")?,
            UserRanks::InternationalMaster => out.write_all(b"InternationalMaster")?,
            UserRanks::Grandmaster => out.write_all(b"Grandmaster")?,
            UserRanks::InternationalGrandmaster => out.write_all(b"InternationalGrandmaster")?,
            UserRanks::LegendaryGrandmaster => out.write_all(b"LegendaryGrandmaster")?,
            UserRanks::Newbie => out.write_all(b"Newbie")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<diesel::sql_types::Text, Pg> for UserRanks {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Pupil" => Ok(UserRanks::Pupil),
            b"Speacialist" => Ok(UserRanks::Speacialist),
            b"Expert" => Ok(UserRanks::Expert),
            b"CandidateMaster" => Ok(UserRanks::CandidateMaster),
            b"Master" => Ok(UserRanks::Master),
            b"InternationalMaster" => Ok(UserRanks::InternationalMaster),
            b"Grandmaster" => Ok(UserRanks::Grandmaster),
            b"InternationalGrandmaster" => Ok(UserRanks::InternationalGrandmaster),
            b"LegendaryGrandmaster" => Ok(UserRanks::LegendaryGrandmaster),
            b"Newbie" => Ok(UserRanks::Newbie),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

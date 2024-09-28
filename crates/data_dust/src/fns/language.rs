use crate::fns::DbError;
use crate::models::Language;
use crate::schema::languages;
use diesel::prelude::*;
use diesel::PgConnection;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Languages {
    languages: HashMap<String, Language>,
}

impl Languages {
    pub fn new(conn: &mut PgConnection) -> Result<Self, DbError> {
        let languages = crate::fns::language::get_all_languages(conn)?;
        let mut language_map = HashMap::new();
        for lang in languages {
            let key = format!("{}_{}", lang.name, lang.version);
            language_map.insert(key, lang);
        }
        Ok(Self {
            languages: language_map,
        })
    }

    pub fn get(&self, name: &str, version: &str) -> Option<&Language> {
        let key = format!("{name}_{version}");
        self.languages.get(&key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Language> {
        self.languages.values()
    }
}

lazy_static! {
    pub static ref ALL_LANGUAGES: Languages = {
        let mut conn = crate::fns::initialize_db_pool()
            .get()
            .expect("Failed to get DB connection");
        Languages::new(&mut conn).expect("Failed to initialize languages")
    };
}

pub fn get_all_languages(conn: &mut PgConnection) -> Result<Vec<Language>, DbError> {
    languages::table
        .load::<Language>(conn)
        .map_err(|e| e.into())
}

pub fn get_language_by_name_and_version(
    conn: &mut PgConnection,
    name: &str,
    version: &str,
) -> Result<Language, DbError> {
    languages::table
        .filter(languages::name.eq(name))
        .filter(languages::version.eq(version))
        .first::<Language>(conn)
        .map_err(|e| e.into())
}

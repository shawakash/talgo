pub mod user;

pub struct AppState {
    pub db_pool: data_dust::fns::DbPool,
}

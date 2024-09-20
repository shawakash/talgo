use std::io::Error;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use routes::user::user_config;
use serde_json::json;

pub mod routes;
pub mod types;
use crate::routes::contest::contest_config;
use crate::types::AppState;

use data_dust::fns::initialize_db_pool;

async fn root() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "Ok"
    }))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        db_pool: initialize_db_pool(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(|cfg| contest_config(cfg, &app_state))
            .configure(|cfg| user_config(cfg, &app_state))
            .route("/", web::get().to(root))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

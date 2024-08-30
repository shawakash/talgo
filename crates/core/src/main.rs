use std::io::Error;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use serde_json::json;

use data_dust::fns::initialize_db_pool;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok().json(json!({
                        "status": "Ok"
                    }))
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

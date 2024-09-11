use crate::types::AppState;
use actix_web::{web, HttpResponse, Responder};

async fn login() -> impl Responder {
    HttpResponse::Ok().body("login")
}

pub fn user_config(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(web::scope("/user").service(web::resource("/login").route(web::post().to(login))));
}

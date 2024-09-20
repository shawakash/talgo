use crate::types::{
    user::{LoginReq, LoginRes},
    AppState,
};
use actix_web::cookie::{time::OffsetDateTime, Cookie, SameSite};
use actix_web::{web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use data_dust::fns::user::{encrypt_token, get_user_id_by_username_email, verify_password};
use uuid::Uuid;

async fn login(app_state: web::Data<AppState>, login_req: web::Json<LoginReq>) -> impl Responder {
    HttpResponse::Ok().json(LoginRes {
        success: true,
        message: "Login successful".to_string(),
        user_id: None,
    })
}

async fn signup(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("signup")
}

async fn user_contests(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Contests for user {}", user_id))
}

async fn user_submissions(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Submissions for user {}", user_id))
}

async fn user_profile(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Profile for user {}", user_id))
}

pub fn user_config(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/user")
            .service(
                web::resource("/login")
                    .app_data(app_state.clone())
                    .route(web::post().to(login)),
            )
            .service(
                web::resource("/signup")
                    .app_data(app_state.clone())
                    .route(web::post().to(signup)),
            )
            .service(
                web::scope("/{id}")
                    .service(
                        web::resource("/contest")
                            .app_data(app_state.clone())
                            .route(web::get().to(user_contests)),
                    )
                    .service(
                        web::resource("/submissions")
                            .app_data(app_state.clone())
                            .route(web::get().to(user_submissions)),
                    )
                    .service(
                        web::resource("")
                            .app_data(app_state.clone())
                            .route(web::get().to(user_profile)),
                    ),
            ),
    );
}

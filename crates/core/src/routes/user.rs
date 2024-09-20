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
    let mut conn = app_state.db_pool.get().unwrap();

    let login_result = web::block(move || {
        if let Ok((user_id, hash)) =
            get_user_id_by_username_email(&mut conn, &login_req.username_or_email)
        {
            if verify_password(&login_req.password, &hash).unwrap() {
                LoginRes {
                    success: true,
                    message: "Logged in!".to_string(),
                    user_id: Some(user_id.to_string()),
                }
            } else {
                LoginRes {
                    success: false,
                    message: "Password Mismatch".to_string(),
                    user_id: None,
                }
            }
        } else {
            LoginRes {
                success: false,
                message: "No such user".to_string(),
                user_id: None,
            }
        }
    })
    .await
    .unwrap();

    let mut response = HttpResponse::Ok().json(&login_result);

    if login_result.success {
        if let Some(user_id) = &login_result.user_id {
            let session_id = Uuid::new_v4().to_string();
            let expiration = Utc::now() + Duration::hours(24);

            let session_token = format!("{}:{}:{}", user_id, session_id, expiration.timestamp());
            match encrypt_token(&session_token) {
                Ok(encrypted_token) => {
                    let cookie_expiration =
                        OffsetDateTime::from_unix_timestamp(expiration.timestamp()).unwrap();

                    let cookie = Cookie::build("session", encrypted_token)
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .same_site(SameSite::Strict)
                        .expires(cookie_expiration)
                        .max_age(actix_web::cookie::time::Duration::hours(24))
                        .finish();

                    response.add_cookie(&cookie).unwrap();

                    // Store session info in cache
                    // store_session(user_id, session_id, expiration).await;
                }
                Err(_) => {
                    response = HttpResponse::InternalServerError().json(LoginRes {
                        success: false,
                        message: "Session creation failed".to_string(),
                        user_id: None,
                    });
                }
            }
        }
    }

    response
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

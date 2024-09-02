use crate::types::AppState;
use actix_web::web;
use submit::submit;

// contest/{id}/submit
pub mod submit;

pub fn contest_config(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/contest").service(
            web::scope("/{id}").service(
                web::resource("/submit")
                    .app_data(app_state.clone())
                    .route(web::post().to(submit)),
            ),
        ),
    );
}

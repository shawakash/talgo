use actix_web::{error::ErrorInternalServerError, web, HttpResponse, Result};
use data_dust::fns::submit::insert_submit;
use data_dust::types::submit::NewSubmission;
use serde_json::json;

pub async fn submit(
    state: web::Data<crate::types::AppState>,
    submit_request: web::Json<NewSubmission>,
) -> Result<HttpResponse> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let sub_id = web::block(move || {
        insert_submit(&mut conn, submit_request.into_inner())
            .map_err(|e| format!("Submission error: {}", e))
    })
    .await
    .map_err(|e| ErrorInternalServerError(e.to_string()))?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "Ok",
        "id": sub_id,
    })))
}

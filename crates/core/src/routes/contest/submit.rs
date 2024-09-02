use actix_web::{web, HttpResponse, Result, error::ErrorInternalServerError};
use data_dust::types::submit::NewSubmission;
use data_dust::fns::submit::insert_submit;
use serde_json::json;

pub async fn submit(
    state: web::Data<crate::types::AppState>,
    submit_request: web::Json<NewSubmission>,
) -> Result<HttpResponse> {
    let sub_id = web::block(move || {
        let mut conn = state.db_pool.get().map_err(|e| format!("Database error: {}", e))?;
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

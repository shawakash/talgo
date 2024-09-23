use data_dust::models::Submissions;
use redis::Pipeline;
use serde_json::json;
use std::sync::Arc;
use tokio::task;

pub async fn publish_submissions_to_channel(
    redis_client: Arc<redis::Client>,
    submissions: Vec<Submissions>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    task::spawn(async move {
        let mut conn = redis_client.get_multiplexed_async_connection().await?;
        let mut pipe = Pipeline::new();

        for submission in submissions {
            let channel = format!("submission_{}", submission.id);
            let message = json!({
                "id": submission.id,
                "verdict": "InQueue",
                "status": submission.status
            })
            .to_string();

            pipe.publish(channel, message);
        }

        pipe.query_async(&mut conn).await?;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await??;

    Ok(())
}

use crate::socket::publish_submissions_to_channel;
use data_dust::enums::sub::SubmissionVerdict;
use data_dust::models::Submissions;
use data_dust::producer::KafkaProducer;
use data_dust::types::queue::QueueMessage;
use diesel::prelude::*;
use diesel::sql_query;
use redis::Client as RedisClient;
use std::sync::Arc;

pub async fn process_submissions(
    conn: &mut PgConnection,
    producer: &KafkaProducer,
    redis_client: Arc<RedisClient>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let batch_submission_cnt = 100;
    let submissions =
        data_dust::fns::submit::get_last_n_pending_submissions(conn, batch_submission_cnt)?;

    if submissions.is_empty() {
        return Ok(());
    }

    let mut messages_by_language: std::collections::HashMap<
        String,
        Vec<(String, QueueMessage<Submissions>)>,
    > = std::collections::HashMap::new();

    for submission in &submissions {
        println!("Put {}", submission.id);

        let queue_message = QueueMessage {
            message_type: "submission".to_string(),
            payload: submission.clone(),
        };

        messages_by_language
            .entry(submission.language.clone())
            .or_insert_with(Vec::new)
            .push((submission.id.to_string(), queue_message));
    }

    let mut all_successful = true;

    for (language, messages) in messages_by_language {
        let topic = format!("submissions_{}", language.to_lowercase());
        let produce_results = producer.produce_batch(&topic, messages).await;

        if !produce_results.iter().all(|r| r.is_ok()) {
            all_successful = false;
            println!(
                "Error: Some messages failed to produce for language: {}",
                language
            );
        }
    }

    if all_successful {
        let verdict_updates: Vec<(i64, SubmissionVerdict)> = submissions
            .iter()
            .map(|s| (s.id, SubmissionVerdict::InQueue))
            .collect();
        data_dust::fns::submit::update_multiple_submission_verdicts(conn, verdict_updates)?;

        publish_submissions_to_channel(redis_client, submissions).await?;
    } else {
        println!("Error: Some messages failed to produce. Submissions were not updated.");
    }

    Ok(())
}

pub async fn process_pending_submissions(
    _worker_id: usize,
    db_pool: Arc<data_dust::fns::DbPool>,
    producer: Arc<KafkaProducer>,
    redis_client: Arc<RedisClient>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = db_pool.get()?;

    let lock_id = 1234;
    sql_query("SELECT pg_advisory_lock($1)")
        .bind::<diesel::sql_types::Integer, _>(lock_id)
        .execute(&mut conn)?;

    let result = process_submissions(&mut conn, &producer, redis_client).await;

    sql_query("SELECT pg_advisory_unlock($1)")
        .bind::<diesel::sql_types::Integer, _>(lock_id)
        .execute(&mut conn)?;

    result
}

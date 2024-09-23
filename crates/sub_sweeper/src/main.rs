use data_dust::enums::sub::SubmissionVerdict;
use data_dust::types::queue::{QueueMessage, SubmissionPayload};
use data_dust::{fns::initialize_db_pool, producer::KafkaProducer};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::{self};

async fn process_pending_submissions(
    db_pool: Arc<data_dust::fns::DbPool>,
    producer: Arc<KafkaProducer>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = db_pool.get()?;
    let submissions = data_dust::fns::submit::get_last_10_pending_submissions(&mut conn)?;

    if submissions.is_empty() {
        return Ok(());
    }

    let mut messages_by_language: std::collections::HashMap<
        String,
        Vec<(String, QueueMessage<SubmissionPayload>)>,
    > = std::collections::HashMap::new();

    for submission in &submissions {
        let payload = SubmissionPayload {
            submission_id: submission.id,
            user_id: submission.user_id,
            problem_id: submission.problem_id,
            language: submission.language.clone(),
            contest_id: submission.contest_id,
        };

        let queue_message = QueueMessage {
            message_type: "submission".to_string(),
            payload,
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

        data_dust::fns::submit::update_multiple_submission_verdicts(&mut conn, verdict_updates)?;
    } else {
        println!("Error: Some messages failed to produce. Submissions were not updated.");
    }

    Ok(())
}

async fn worker(
    id: usize,
    db_pool: Arc<data_dust::fns::DbPool>,
    producer: Arc<KafkaProducer>,
    mut shutdown: broadcast::Receiver<()>,
) {
    println!("Worker {} started", id);

    loop {
        tokio::select! {
            Ok(_) = shutdown.recv() => {
                println!("Worker {} shutting down", id);
                break;
            }
            _ = process_pending_submissions(Arc::clone(&db_pool), Arc::clone(&producer)) => {
                println!("Worker {} processed pending submissions", id);
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_pool = Arc::new(initialize_db_pool());
    println!("den");

    let kafka_brokers = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set!");
    let producer = Arc::new(KafkaProducer::new(&kafka_brokers)?);

    let num_workers = env::var("NUM_SWEEPER_BROKERS")
        .unwrap_or_else(|_| "4".to_string())
        .parse::<usize>()
        .expect("NUM_SWEEPER_BROKERS must be a number");

    println!("Starting {} submission workers...", num_workers);

    let (shutdown_sender, _) = broadcast::channel::<()>(1);
    let mut worker_handles = Vec::new();

    for i in 0..num_workers {
        let db_pool = db_pool.clone();
        let producer = producer.clone();
        let shutdown_rx = shutdown_sender.subscribe();

        let handle = tokio::spawn(async move {
            worker(i, db_pool, producer, shutdown_rx).await;
        });
        worker_handles.push(handle);
    }

    println!("All workers started. Press Ctrl+C to stop.");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down...");
        }
    }

    let _ = shutdown_sender.send(());

    for handle in worker_handles {
        handle.await?;
    }

    println!("All workers have shut down. Goodbye!");

    Ok(())
}

use data_dust::producer::KafkaProducer;
use redis::Client as RedisClient;
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast;

use crate::process::process_pending_submissions;

pub async fn worker(
    id: usize,
    db_pool: Arc<data_dust::fns::DbPool>,
    producer: Arc<KafkaProducer>,
    redis_client: Arc<RedisClient>,
    mut shutdown: broadcast::Receiver<()>,
) {
    println!("Worker {} started", id);

    loop {
        tokio::select! {
            Ok(_) = shutdown.recv() => {
                println!("Worker {} shutting down", id);
                break;
            }
            result = process_pending_submissions(id, Arc::clone(&db_pool), Arc::clone(&producer), Arc::clone(&redis_client)) => {
                match result {
                    Ok(_) => {},
                    Err(e) => eprintln!("Worker {} encountered an error: {}", id, e),
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

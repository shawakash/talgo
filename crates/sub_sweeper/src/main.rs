use data_dust::fns::check_db_connection;
use data_dust::{fns::initialize_db_pool, producer::KafkaProducer};
use dotenvy::dotenv;
use redis::Client as RedisClient;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::{self};
use worker::worker;

mod process;
mod socket;
mod worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_pool = Arc::new(initialize_db_pool());
    if !check_db_connection(&db_pool).await {
        eprintln!("Failed to connect to the database. Exiting.");
        return Ok(());
    }
    println!("Database connection successful.");

    let kafka_brokers = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set!");
    let producer = Arc::new(KafkaProducer::new(&kafka_brokers)?);

    if !producer.check_connection() {
        eprintln!("Failed to connect to Kafka. Exiting.");
        return Ok(());
    }
    println!("Kafka connection successful.");

    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string())
        .parse::<String>()
        .expect("REDIS_URL must be set!");
    let redis_client = Arc::new(RedisClient::open(redis_url)?);

    let mut redis_conn = redis_client.get_multiplexed_async_connection().await?;
    redis::cmd("PING").query_async(&mut redis_conn).await?;
    println!("Redis connection successful.");

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
        let redis_client = redis_client.clone();
        let shutdown_rx = shutdown_sender.subscribe();

        let handle = tokio::spawn(async move {
            worker(i, db_pool, producer, redis_client, shutdown_rx).await;
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

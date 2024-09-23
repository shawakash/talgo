use futures::future::{self, FutureExt};
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::ClientConfig;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

pub struct KafkaProducer {
    producer: Arc<FutureProducer>,
}

#[derive(Debug)]
pub enum ProducerError {
    Kafka(rdkafka::error::KafkaError),
    Serialization(serde_json::Error),
}

impl From<rdkafka::error::KafkaError> for ProducerError {
    fn from(err: rdkafka::error::KafkaError) -> Self {
        ProducerError::Kafka(err)
    }
}

impl From<serde_json::Error> for ProducerError {
    fn from(err: serde_json::Error) -> Self {
        ProducerError::Serialization(err)
    }
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Result<Self, rdkafka::error::KafkaError> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("batch.size", "16384")
            .set("linger.ms", "5")
            .create()?;

        Ok(Self {
            producer: Arc::new(producer),
        })
    }

    pub async fn produce<T: Serialize + Send + 'static>(
        &self,
        topic: &str,
        key: String,
        value: T,
    ) -> Result<(), ProducerError> {
        let payload = serde_json::to_string(&value)?;

        self.producer
            .send(
                FutureRecord::to(topic).payload(&payload).key(&key),
                Duration::from_secs(0),
            )
            .await
            .map(|_| ())
            .map_err(|(e, _)| e)?;

        Ok(())
    }

    pub async fn produce_batch<T: Serialize + Send + 'static>(
        &self,
        topic: &str,
        messages: Vec<(String, T)>,
    ) -> Vec<Result<(), ProducerError>> {
        let futures = messages.into_iter().map(|(key, value)| {
            let producer = Arc::clone(&self.producer);
            let topic = topic.to_string();
            async move {
                let payload = serde_json::to_string(&value)?;
                producer
                    .send(
                        FutureRecord::to(&topic).payload(&payload).key(&key),
                        Duration::from_secs(0),
                    )
                    .await
                    .map(|_| ())
                    .map_err(|(e, _)| e)?;
                Ok(())
            }
            .boxed()
        });

        future::join_all(futures).await
    }

    pub async fn produce_multiple<T: Serialize + Send + 'static>(
        &self,
        topic: &str,
        messages: Vec<(String, T)>,
    ) -> Vec<Result<(), ProducerError>> {
        let mut results = Vec::with_capacity(messages.len());

        for (key, value) in messages {
            let result = self.produce(topic, key, value).await;
            results.push(result);
        }

        results
    }

    pub fn check_connection(&self) -> bool {
        match self
            .producer
            .client()
            .fetch_metadata(None, Duration::from_secs(5))
        {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to connect to Kafka: {}", e);
                false
            }
        }
    }
}

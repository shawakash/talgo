use futures::stream::StreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::Message,
    ClientConfig, TopicPartitionList,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;

pub struct KafkaConsumer {
    consumer: Arc<StreamConsumer>,
    topic: String,
}

#[derive(Debug)]
pub enum ConsumerError {
    Kafka(rdkafka::error::KafkaError),
    Deserialization(serde_json::Error),
}

impl From<rdkafka::error::KafkaError> for ConsumerError {
    fn from(err: rdkafka::error::KafkaError) -> Self {
        ConsumerError::Kafka(err)
    }
}

impl From<serde_json::Error> for ConsumerError {
    fn from(err: serde_json::Error) -> Self {
        ConsumerError::Deserialization(err)
    }
}

impl KafkaConsumer {
    pub fn new(
        brokers: &str,
        group_id: &str,
        topic: &str,
    ) -> Result<Self, rdkafka::error::KafkaError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.auto.commit", "false") // Disable auto-commit for manual acknowledgement
            .set("auto.offset.reset", "earliest")
            .set("session.timeout.ms", "6000")
            .create()?;

        consumer.subscribe(&[topic])?;

        Ok(Self {
            consumer: Arc::new(consumer),
            topic: topic.to_string(),
        })
    }

    pub async fn consume<T: DeserializeOwned + Send + 'static>(
        &self,
        max_messages: usize,
        timeout: Duration,
    ) -> Result<Vec<(T, i32, i64)>, ConsumerError> {
        let mut messages = Vec::new();
        let mut message_stream = self.consumer.stream();

        let deadline = tokio::time::Instant::now() + timeout;

        while messages.len() < max_messages {
            match tokio::time::timeout_at(deadline, message_stream.next()).await {
                Ok(Some(message_result)) => match message_result {
                    Ok(borrowed_message) => {
                        if let Some(payload) = borrowed_message.payload() {
                            match serde_json::from_slice::<T>(payload) {
                                Ok(deserialized_payload) => {
                                    let partition = borrowed_message.partition();
                                    let offset = borrowed_message.offset();
                                    messages.push((deserialized_payload, partition, offset));
                                }
                                Err(e) => return Err(ConsumerError::Deserialization(e)),
                            }
                        }
                    }
                    Err(e) => return Err(ConsumerError::Kafka(e)),
                },
                Ok(None) => break,
                Err(_) => break, // Timeout reached
            }
        }

        Ok(messages)
    }

    pub async fn acknowledge(&self, partition: i32, offset: i64) -> Result<(), ConsumerError> {
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(&self.topic, partition, rdkafka::Offset::Offset(offset + 1))?;
        self.consumer.store_offsets(&tpl)?;
        self.consumer
            .commit(&tpl, rdkafka::consumer::CommitMode::Async)?;
        Ok(())
    }

    pub async fn acknowledge_multiple(
        &self,
        offsets: Vec<(i32, i64)>,
    ) -> Result<(), ConsumerError> {
        let mut tpl = TopicPartitionList::new();
        for (partition, offset) in offsets {
            tpl.add_partition_offset(&self.topic, partition, rdkafka::Offset::Offset(offset + 1))?;
        }
        self.consumer.store_offsets(&tpl)?;
        self.consumer
            .commit(&tpl, rdkafka::consumer::CommitMode::Async)?;
        Ok(())
    }
}

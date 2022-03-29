#![allow(dead_code)]

use crate::config::KafkaSettings;
use log::{debug, error};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

pub struct KafkaClient {
    settings: KafkaSettings,
    pub(crate) producer: FutureProducer,
}

impl<'a> KafkaClient {
    pub fn new(settings: KafkaSettings) -> KafkaClient {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &settings.servers)
            .set("message.timeout.ms", settings.timeout_ms.to_string())
            .create()
            .expect("Producer creation error");

        KafkaClient { settings, producer }
    }
}

pub async fn send_kafka_message(
    producer: FutureProducer,
    topic: String,
    key: String,
    raw: &[u8],
) -> bool {
    let record = FutureRecord::to(&topic).key(&key).payload(raw);

    let produce_future = producer.send(record, Duration::from_millis(1)).await;
    return match produce_future {
        Ok(delivery) => {
            debug!("Sent kafka message: {:?}", delivery);
            true
        }
        Err((e, _)) => {
            println!("Error kafka message: {}", String::from_utf8_lossy(raw));
            error!("Error sending kafka message: {:?}", e);
            false
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use crate::utils::init;
    use std::any::TypeId;

    #[test]
    fn test_create_producer() {
        let client = KafkaClient::new(KafkaSettings::default());
        assert_eq!(
            TypeId::of::<FutureProducer>(),
            utils::get_type_of(&client.producer)
        );
    }

    #[tokio::test]
    async fn test_message() {
        init();
        let j = KafkaClient::new(KafkaSettings::default());
        let out = send_kafka_message(
            j.producer,
            "test".to_string(),
            "test".to_string(),
            "hello kafka".as_bytes(),
        )
        .await;
        assert!(out);
    }
}

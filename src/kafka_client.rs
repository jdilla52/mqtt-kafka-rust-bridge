use log::{debug, error};
use std::convert::TryFrom;
use std::error::Error;
use std::time::Duration;

use crate::config::KafkaSettings;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, Message};

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
            debug!("Sent: {:?}", delivery);
            true
        }
        Err((e, _)) => {
            println!("error kafka message: {}", String::from_utf8_lossy(raw));
            error!("Error: {:?}", e);
            false
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::init;
    use std::any::TypeId;

    fn get_type_of<T: 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    #[test]
    fn test_create_producer() {
        let client = KafkaClient::new(KafkaSettings {
            servers: "127.0.0.1:9092".into(),
            timeout_ms: 5000,
            kafka_topic: "".to_string(),
        });
        assert_eq!(
            TypeId::of::<FutureProducer>(),
            get_type_of(&client.producer)
        );
    }

    #[tokio::test]
    async fn test_message() {
        init();
        let j = KafkaClient::new(KafkaSettings {
            servers: "127.0.0.1:9092".into(),
            timeout_ms: 5000,
            kafka_topic: "".to_string(),
        });
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

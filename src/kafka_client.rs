use log::{debug, error};
use std::convert::TryFrom;
use std::error::Error;
use std::time::Duration;

use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, Message};

#[derive(Clone)]
pub struct KafkaClient {
    brokers: String,
    pub(crate) producer: FutureProducer,
}

impl<'a> KafkaClient {
    pub fn new(brokers: String) -> KafkaClient {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        KafkaClient {
            brokers: brokers,
            producer,
        }
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
    use std::any::TypeId;
    use crate::utils::init;

    fn get_type_of<T: 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    #[test]
    fn test_create_producer() {
        let client = KafkaClient::new("127.0.0.1:9092".into());
        assert_eq!(
            TypeId::of::<FutureProducer>(),
            get_type_of(&client.producer)
        );
    }

    #[tokio::test]
    async fn test_message() {
        init();
        let j = KafkaClient::new("127.0.0.1:9092".into());
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

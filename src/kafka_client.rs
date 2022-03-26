use std::error::Error;
use std::convert::TryFrom;
use std::time::Duration;
use log::{debug, error};

use rdkafka::consumer::{StreamConsumer};
use rdkafka::{ClientConfig, Message};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::Consumer;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;

pub struct KafkaClient {
    brokers: String,
    producer: FutureProducer,
}

impl KafkaClient {
    pub fn new(brokers: String) -> KafkaClient {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        KafkaClient {
            brokers,
            producer,
        }
    }

    pub async fn send_message(&self, topic: String, key: String, record: &[u8]) -> bool {
        let record = FutureRecord::to(&topic)
            .key(&key)
            .payload(record);

        let produce_future = producer.send(record, Duration::from_millis(1)).await;
        return match produce_future {
            Ok(delivery) => {
                debug!("Sent: {:?}", delivery);
                true
            }
            Err((e, _)) => {
                error!("Error: {:?}", e);
                false
            }
        };
    }
}

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use super::*;


    fn get_type_of<T: 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    #[test]
    fn test_create_producer() {
        let client = KafkaClient::new("127.0.0.1:9092".into());
        assert_eq!(TypeId::of::<FutureProducer>(), get_type_of(&client.producer));
    }

    #[tokio::test]
    async fn test_message() {
        init();
        let j = KafkaClient::new("127.0.0.1:9092".into());
        let out = j.send_message("test".to_string(), "test".to_string(), "hello kafka".as_bytes()).await;
        assert!(out);
    }
}
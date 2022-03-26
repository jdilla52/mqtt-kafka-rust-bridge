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

struct KafkaClient {
    brokers: String,
    producer: FutureProducer
}

impl KafkaClient {
    pub async fn send_ready_notification(&self, topic: String, record: String)
                                         -> Result<(), Box<dyn Error + Send + Sync> >{

        let record = FutureRecord::to(&topic)
            .key("some key".into())
            .payload(&record);

        let produce_future = self.producer.send(record,Duration::from_millis(1)).await;
        match produce_future {
            Ok(delivery) => debug!("Sent: {:?}", delivery),
            Err((e, _))=> error!("Error: {:?}", e),
        }
        Ok(())
    }

    pub fn new (brokers: String, ) -> KafkaClient {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");


        KafkaClient{
            brokers,
            producer
        }
    }
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
        let j = KafkaClient::new("127.0.0.1:9092".into());
    }

    #[tokio::test]
    async fn test_connection() {

    }
}
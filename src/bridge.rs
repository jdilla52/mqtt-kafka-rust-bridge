use crate::kafka_client::{send_kafka_message, KafkaClient};
use crate::mqtt_client::MqttClient;
use futures::StreamExt;
use std::process;
use std::time::Duration;
use log::{debug, error};
use rdkafka::producer::FutureRecord;

pub struct Bridge {}

impl Bridge {
    pub async fn new() {
        let mut mqtt_client =
            MqttClient::new("tcp://127.0.0.1:1883".to_string(), "test".to_string());
        mqtt_client.build_mqtt_connection().await;
        let valid = mqtt_client.subscribe().await;

        let kafka_client = KafkaClient::new("127.0.0.1:9092".into());
        let mut stream = mqtt_client.message_stream.unwrap_or_else(|| {
            println!("cant reconnect without an mqtt client please create a client");
            process::exit(1);
        });

        while let Some(msg_opt) = stream.next().await {
            if let Some(msg) = msg_opt {
                let l = kafka_client.producer.clone();
                tokio::spawn(async move {
                    // https://docs.rs/rdkafka/0.28.0/rdkafka/producer/struct.FutureProducer.html
                    // docs: clone producer to threads
                    send_kafka_message(l, "test".into(), "lkjl".into(), msg.payload()).await;
                });
            }
            else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                // mqtt_client.try_reconnect();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;
    use crate::utils::init;

    fn get_type_of<T: 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }


    #[tokio::test]
    async fn test_bridge_message() {
        init();
        let j = Bridge::new().await;
    }

    // #[test]
    // fn test_create_producer() {
    //     let client = KafkaClient::new("127.0.0.1:9092".into());
    //     assert_eq!(
    //         TypeId::of::<FutureProducer>(),
    //         get_type_of(&client.producer)
    //     );
    // }
}
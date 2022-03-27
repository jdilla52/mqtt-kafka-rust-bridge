use crate::config::BridgeSettings;
use crate::kafka_client::{send_kafka_message, KafkaClient};
use crate::mqtt_client::MqttClient;
use futures::StreamExt;
use log::{debug, error};
use rdkafka::producer::FutureRecord;
use std::process;
use std::time::Duration;
use uuid::Uuid;

pub struct Bridge {
    config: BridgeSettings,
    mqtt_client: MqttClient,
    kafka_client: KafkaClient
}

fn mqtt_to_kafka_topic(v: &str) -> String {
    str::replace(v, "/", "-")
}

impl Bridge {
    pub async fn new(settings: BridgeSettings) {
        let mut mqtt_client = MqttClient::new(settings.mqtt_settings).await;
        let valid = mqtt_client.subscribe().await; // maybe move
        let kafka_client = KafkaClient::new(settings.kafka_settings);
    }
    pub async fn run(&mut self) {
        while let Some(msg_opt) = self.mqtt_client.message_stream.next().await {
            if let Some(msg) = msg_opt {
                let l = self.kafka_client.producer.clone();

                tokio::spawn(async move {
                    // generate kafka topic from mqtt topic
                    let topic = mqtt_to_kafka_topic(msg.topic());
                    // using uuid as kafka message key
                    let uuid = Uuid::new_v4();
                    // https://docs.rs/rdkafka/0.28.0/rdkafka/producer/struct.FutureProducer.html
                    // docs: clone producer to threads
                    send_kafka_message(l, topic, uuid.to_string(), msg.payload()).await;
                });
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                self.mqtt_client.try_reconnect();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::init;
    use std::any::TypeId;

    fn get_type_of<T: 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    #[tokio::test]
    async fn test_bridge_message() {
        init();
        let j = Bridge::new(BridgeSettings::default()).await;
    }

    // #[tokio::test]
    // async fn test_bridge_message() {
    //     init();
    //     let j = Bridge::new().await;
    // }

    // #[test]
    // fn test_create_producer() {
    //     let client = KafkaClient::new("127.0.0.1:9092".into());
    //     assert_eq!(
    //         TypeId::of::<FutureProducer>(),
    //         get_type_of(&client.producer)
    //     );
    // }
}

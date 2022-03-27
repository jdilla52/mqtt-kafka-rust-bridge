#![allow(dead_code)]

use crate::config::BridgeSettings;
use crate::kafka_client::{send_kafka_message, KafkaClient};
use crate::mqtt_client::MqttClient;
use futures::StreamExt;
use log::{debug, error};
use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct BridgeStats {
    skipped_messages: i32,
    routed_messages: i32,
    errors: i32,
    connection_error: i32,
    start_time: SystemTime,
}

impl Default for BridgeStats {
    fn default() -> Self {
        BridgeStats {
            skipped_messages: 0,
            routed_messages: 0,
            errors: 0,
            connection_error: 0,
            start_time: SystemTime::now(),
        }
    }
}

pub struct Bridge {
    settings: BridgeSettings,
    mqtt_client: MqttClient,
    kafka_client: KafkaClient,
    bridge_stats: Arc<Mutex<BridgeStats>>,
}

fn mqtt_to_kafka_topic(v: &str) -> String {
    str::replace(v, "/", "-")
}

impl Bridge {
    pub async fn new(settings: BridgeSettings) -> Bridge {
        let mqtt_client = MqttClient::new(settings.mqtt_settings.clone()).await;
        mqtt_client.subscribe().await; // maybe move
        let kafka_client = KafkaClient::new(settings.kafka_settings.clone());

        Bridge {
            mqtt_client,
            kafka_client,
            settings,
            bridge_stats: Arc::new(Mutex::new(BridgeStats::default())),
        }
    }
    pub async fn run(&mut self) {
        while let Some(msg_opt) = self.mqtt_client.message_stream.next().await {
            if let Some(msg) = msg_opt {
                // clone just the data we need in the threads
                let kafka_producer = self.kafka_client.producer.clone();
                let kafka_topic = self.settings.kafka_settings.kafka_topic.clone();
                let stats = Arc::clone(&self.bridge_stats);
                tokio::spawn(async move {
                    let mut guard = stats.lock().await;
                    if kafka_topic != "*" {
                        // only allow wild card for rn
                        guard.skipped_messages += 1;
                    }
                    guard.routed_messages += 1;
                    drop(guard);

                    // generate kafka topic from mqtt topic
                    let topic = mqtt_to_kafka_topic(msg.topic());
                    // using uuid as kafka message key
                    let uuid = Uuid::new_v4();

                    // docs: clone producer to threads : https://docs.rs/rdkafka/0.28.0/rdkafka/producer/struct.FutureProducer.html
                    let success =
                        send_kafka_message(kafka_producer, topic, uuid.to_string(), msg.payload())
                            .await;
                    if !success {
                        let mut guard = stats.lock().await;
                        guard.errors += 1;
                        drop(guard);
                    }
                });
            } else {
                let mut guard = self.bridge_stats.lock().await;
                guard.connection_error += 1;
                drop(guard);
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
        Bridge::new(BridgeSettings::default()).await;
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

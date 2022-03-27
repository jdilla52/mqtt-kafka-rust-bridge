extern crate confy;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct MqttSettings {
    address: String,
    client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KafkaSettings {
    servers: String,
    timeout_ms: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TopicSettings {
    mqtt_topic: String,
    kafka_topic: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BridgeSettings {
    mqtt_settings: MqttSettings,
    kafka_settings: KafkaSettings,
    topic_settings: TopicSettings,
}

impl Default for BridgeSettings {
    fn default() -> Self {
        BridgeSettings{
            mqtt_settings: MqttSettings { address: "tcp://127.0.0.1:1883".to_string(), client_id: "test_client".to_string() },
            kafka_settings: KafkaSettings { servers: "127.0.0.1:9092".to_string(), timeout_ms: 0 },
            topic_settings: TopicSettings { mqtt_topic: "*".to_string(), kafka_topic: "*".to_string() },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{BridgeSettings, MqttSettings, KafkaSettings, TopicSettings};


    #[test]
    fn load_config() {
        let config: Config = confy::load_path("config/.conf").unwrap();
    }
}



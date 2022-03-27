extern crate confy;
extern crate serde;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MqttSettings {
    address: String,
    client_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct KafkaSettings {
    servers: String,
    timeout_ms: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TopicSettings {
    mqtt_topic: String,
    kafka_topic: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct BridgeSettings {
    mqtt_settings: MqttSettings,
    kafka_settings: KafkaSettings,
    topic_settings: TopicSettings,
}

impl BridgeSettings {
    fn as_json(&self)-> String {
        return serde_json::to_string(&self).unwrap()
    }

    fn from_json(s: &str)-> Self {
        return serde_json::from_str(s).unwrap()
    }
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
    fn test_parse(){
        let t = BridgeSettings{
            mqtt_settings: MqttSettings { address: "tcp://127.0.0.1:1883".to_string(), client_id: "test_client".to_string() },
            kafka_settings: KafkaSettings { servers: "127.0.0.1:9092".to_string(), timeout_ms: 0 },
            topic_settings: TopicSettings { mqtt_topic: "*".to_string(), kafka_topic: "*".to_string() },
        };

        assert_eq!(t, BridgeSettings::from_json(&t.as_json()));
    }

    #[test]
    fn load_config() {
        let config: BridgeSettings = confy::load_path("config/.conf").unwrap();
    }
}



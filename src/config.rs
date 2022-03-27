#![allow(dead_code)]

extern crate confy;
extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MqttSettings {
    pub(crate) address: String,
    pub(crate) client_id: String,
    pub(crate) mqtt_topic: Vec<String>,
    pub(crate) mqtt_qos: Vec<i32>,
}

impl Default for MqttSettings {
    fn default() -> Self {
        MqttSettings {
            address: "tcp://127.0.0.1:1883".to_string(),
            client_id: "test_client".to_string(),
            mqtt_topic: vec!["#".to_string()],
            mqtt_qos: vec![1],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct KafkaSettings {
    pub(crate) servers: String,
    pub(crate) timeout_ms: i32,
    pub(crate) kafka_topic: String,
}

impl Default for KafkaSettings {
    fn default() -> Self {
        KafkaSettings {
            servers: "127.0.0.1:9092".to_string(),
            timeout_ms: 5000,
            kafka_topic: "*".to_string(),
        }
    }
}
// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// pub struct TopicSettings {
//     mqtt_topic: Vec<String>,
//     kafka_topic: String,
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BridgeSettings {
    pub mqtt_settings: MqttSettings,
    pub kafka_settings: KafkaSettings,
}

impl BridgeSettings {
    fn as_json(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }

    fn from_json(s: &str) -> Self {
        return serde_json::from_str(s).unwrap();
    }
}

impl Default for BridgeSettings {
    fn default() -> Self {
        BridgeSettings {
            mqtt_settings: MqttSettings {
                address: "tcp://127.0.0.1:1883".to_string(),
                client_id: "test_client".to_string(),
                mqtt_topic: vec!["#".to_string()],
                mqtt_qos: vec![1],
            },
            kafka_settings: KafkaSettings {
                servers: "127.0.0.1:9092".to_string(),
                timeout_ms: 5000,
                kafka_topic: "*".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{BridgeSettings, KafkaSettings, MqttSettings};

    #[test]
    fn test_parse() {
        let t = BridgeSettings {
            mqtt_settings: MqttSettings {
                address: "tcp://127.0.0.1:1883".to_string(),
                client_id: "test_client".to_string(),
                mqtt_topic: vec!["*".to_string()],
                mqtt_qos: vec![1],
            },
            kafka_settings: KafkaSettings {
                servers: "127.0.0.1:9092".to_string(),
                timeout_ms: 0,
                kafka_topic: "*".to_string(),
            },
        };

        assert_eq!(t, BridgeSettings::from_json(&t.as_json()));
    }

    #[test]
    fn load_config() {
        let _config: BridgeSettings = confy::load_path("../config/default.conf").unwrap();
    }
}

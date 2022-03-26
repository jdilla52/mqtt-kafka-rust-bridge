use std::process;
use futures::StreamExt;
use crate::kafka_client::KafkaClient;
use crate::mqtt_client::MqttClient;

struct Bridge {}

impl Bridge {
    pub async fn new() {
        let mut mqtt_client = MqttClient::new(
            "tcp://127.0.0.1:1883".to_string(),
                                     "test".to_string(), );
            mqtt_client.build_mqtt_connection().await;
        let valid = mqtt_client.subscribe().await;

        let kafka_client = KafkaClient::new("127.0.0.1:9092".into());
        let mut stream = mqtt_client.message_stream.unwrap_or_else(|| {
            println!("cant reconnect without an mqtt client please create a client");
            process::exit(1);
        });

        while let Some(msg_opt) = stream.next().await {

            if let Some(msg) = msg_opt {
                tokio::spawn(async move {
                   // kafka_client.send_message("test".into(), "test".into(), msg.payload());
                });
                println!("{}", msg);

            }
            else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                mqtt_client.try_reconnect();
            }
        }

    }
}
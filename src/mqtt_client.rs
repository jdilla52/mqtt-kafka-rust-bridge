use futures::executor::block_on;
use futures::stream::StreamExt;
use log::{error, info};
use paho_mqtt as mqtt;
use paho_mqtt::async_client::AsyncClient;
use paho_mqtt::{AsyncReceiver, ConnectResponse, DeliveryToken, Message, Receiver, ServerResponse};
use std::ptr::addr_of_mut;
use std::time::Duration;
use std::{process, thread};

pub struct MqttClient {
    mqtt_addr: String,
    client_id: String,
    pub cli: AsyncClient,
    pub message_stream: AsyncReceiver<Option<Message>>,
}

impl MqttClient {
    pub async fn new(mqtt_addr: String, client_id: String) -> MqttClient {
        info!("rbot is connecting");
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(mqtt_addr.clone())
            .client_id(client_id.clone())
            .max_buffered_messages(100)
            .finalize();

        let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            eprintln!("Error creating the client: {:?}", e);
            error!("Unable to connect: {:?}", e);
            process::exit(1);
        });

        let ssl_opts = mqtt::SslOptionsBuilder::new()
            .enable_server_cert_auth(false)
            // .trust_store(trust_store)?
            // .key_store(key_store)?
            .finalize();

        // Define the set of options for the connection
        let lwt = mqtt::MessageBuilder::new()
            .topic("lwt")
            .payload("Sync consumer lost connection")
            .finalize();

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .ssl_options(ssl_opts)
            // .user_name("test_user")
            // .password("test_password")
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(false)
            .will_message(lwt)
            .finalize();

        // Get message stream before connecting.
        let mut message_stream = cli.get_stream(1024);
        // self.message_stream = Option::from(stream);

        let rsp: ServerResponse = match cli.connect(conn_opts).await {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to connect: {:?}", mqtt_addr);
                eprintln!("Unable to connect: {:?}", mqtt_addr);
                process::exit(1);
            }
        };

        match rsp.connect_response() {
            Some(conn_rsp) => {
                println!(
                    "Connected to: '{}' with MQTT version {}",
                    conn_rsp.server_uri, conn_rsp.mqtt_version
                );
            }
            _ => {
                println!("existing session");
            }
        }
        MqttClient {
            mqtt_addr,
            client_id,
            cli,
            message_stream,
        }
    }
    pub async fn subscribe(&self) -> bool {
        // add config to ignore or add specific topics
        let subscriptions = &["test"];
        let qos = &[1];

        let resp = self.cli.subscribe_many(subscriptions, qos).await;
        match resp {
            Ok(v) => {
                let r = v.subscribe_many_response();
                return true;
            }
            Err(_e) => {
                error!(
                    "Unable to subscribe: {:?} on topics {:?}",
                    self.mqtt_addr, subscriptions
                );
                eprintln!(
                    "Unable to subscribe: {:?} on topics {:?}",
                    self.mqtt_addr, subscriptions
                );
                process::exit(1);
            }
        }
    }

    pub fn try_reconnect(&self) -> bool {
        println!("Connection lost. Waiting to retry connection");
        for _ in 0..12 {
            thread::sleep(Duration::from_millis(5000));
            if self.cli.reconnect().wait().is_ok() {
                println!("Successfully reconnected");
                return true;
            }
        }
        println!("Unable to reconnect after several attempts.");
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    fn get_type_of<T: 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    #[tokio::test]
    async fn test_connection() {
        let mut client =
            MqttClient::new("tcp://127.0.0.1:1883".to_string(), "test".to_string()).await;
        assert_eq!(TypeId::of::<AsyncClient>(), get_type_of(&client.cli));
    }

    #[tokio::test]
    async fn test_reconnect() {
        let mut client =
            MqttClient::new("tcp://127.0.0.1:1883".to_string(), "test".to_string()).await;
        client.cli.disconnect(None);
        let reconnect = client.try_reconnect();
        assert!(reconnect);
    }

    #[tokio::test]
    async fn test_subscription() {
        let mut client =
            MqttClient::new("tcp://127.0.0.1:1883".to_string(), "test".to_string()).await;
        let valid = client.subscribe().await;
        assert!(valid);
    }
}

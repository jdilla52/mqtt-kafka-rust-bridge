use log::{error, info};
use paho_mqtt as mqtt;
use paho_mqtt::async_client::AsyncClient;
use paho_mqtt::{DeliveryToken, Message};
use std::process;

struct MqttClient {
    mqtt_addr: String,
    client_id: String,
}

impl MqttClient {
    pub fn create_mqtt_connection(&self) -> AsyncClient {
        info!("rbot is connecting");
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(self.mqtt_addr.clone())
            .client_id(self.client_id.clone())
            .max_buffered_messages(100)
            .finalize();

        let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            eprintln!("Error creating the client: {:?}", e);
            error!("Unable to connect: {:?}", e);
            process::exit(1);
        });

        let ssl_opts = mqtt::SslOptionsBuilder::new()
            .enable_server_cert_auth(false)
            // .trust_store(trust_store)?
            // .key_store(key_store)?
            .finalize();

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .ssl_options(ssl_opts)
            // .user_name("test_user")
            // .password("test_password")
            // .will_message(will)
            .finalize();

        if let Err(_e) = cli.connect(conn_opts).wait() {
            eprintln!("Unable to connect: {:?}", self.mqtt_addr);
            error!("Unable to connect: {:?}", self.mqtt_addr);
            process::exit(1);
        }
        cli
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let client = MqttClient {
            mqtt_addr: "tcp://localhost::1883".to_string(),
            client_id: "".to_string(),
        };
        let cli = client.create_mqtt_connection();
    }
}

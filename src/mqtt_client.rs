use log::{error, info};
use paho_mqtt as mqtt;
use paho_mqtt::async_client::AsyncClient;
use paho_mqtt::{DeliveryToken, Message};
use std::process;
use std::time::Duration;

struct MqttClient {
    mqtt_addr: String,
    client_id: String,
    cli: Option<AsyncClient>,
}

impl MqttClient {
    pub async fn build_mqtt_connection(&mut self) {
        info!("rbot is connecting");
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(self.mqtt_addr.clone())
            .client_id(self.client_id.clone())
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
            .topic("test")
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



        let subscriptions = ["#"];

        let qos = [1];

        let resp = cli.connect(conn_opts).await;
        // https://github.com/eclipse/paho.mqtt.rust/blob/master/examples/sync_consume.rs
        match resp {
            Ok(rsp) => {
                if let Some(conn_rsp) = rsp.connect_response() {
                    println!(
                        "Connected to: '{}' with MQTT version {}",
                        conn_rsp.server_uri, conn_rsp.mqtt_version
                    );
                    if !conn_rsp.session_present {
                        // Register subscriptions on the server
                        println!("Subscribing to topics with requested QoS: {:?}...", qos);
                        // Initialize the consumer before connecting
                        // cli.subscribe_many(&subscriptions, &qos)
                        //     .and_then(|rsp| {
                        //         rsp.subscribe_many_response()
                        //             .ok_or(mqtt::Error::General("Bad response"))
                        //     })
                        //     .and_then(|vqos| {
                        //         println!("QoS granted: {:?}", vqos);
                        //         Ok(())
                        //     })
                        //     .unwrap_or_else(|err| {
                        //         println!("Error subscribing to topics: {:?}", err);
                        //         cli.disconnect(None).unwrap();
                        //         process::exit(1);
                        //     });
                    }
                    else {
                        println!("using existing session");
                    }
                }
            }
            Err(e) => {
                error!("Unable to connect: {:?}", self.mqtt_addr);
                eprintln!("Unable to connect: {:?}", self.mqtt_addr);
                process::exit(1);
            }
        }
        self.cli = Option::from(cli);
    }
    pub fn subscribe(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_connection() {
        env_logger::init();

        let mut client = MqttClient {
            mqtt_addr: "ssl://localhost:18883".to_string(),
            client_id: "".to_string(),
            cli: None,
        };

        let cli = client.build_mqtt_connection().await;

        // let msg = mqtt::MessRageBuilder::new()
        //     .topic("test")R
        //     .payload("hello")
        //     .qos(1)
        //     .finalize();
        //
        // cli.publish(msg);
    }
}

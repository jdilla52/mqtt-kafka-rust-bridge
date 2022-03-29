mod bridge;
mod config;
mod http_server;
mod kafka_client;
mod mqtt_client;
mod utils;

#[cfg(test)]
mod tests {
    use crate::bridge::Bridge;
    use crate::config::BridgeSettings;

    #[tokio::test]
    async fn run_bridge() {
        let cfg: BridgeSettings = confy::load("../config/default.conf").unwrap();
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        Bridge::new(cfg)
            .await
            // .spawn_api()
            .run()
            .await;
    }
}

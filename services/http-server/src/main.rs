use rsiot_http_server::{cmp_http_server, ComponentChain};
use rsiot_redis_publisher::cmp_redis_publisher;
use rsiot_redis_subscriber::cmp_redis_subscriber;
use tokio::main;
use tracing::Level;

use env_vars::{load_config, Config};
use logging::configure_logging;
use messages::Messages;

#[main]
async fn main() {
    let config = load_config::<Config>().expect("Setting not loaded");

    configure_logging("api", &config.loki_url(), Level::INFO)
        .await
        .expect("Error in logger initialization");

    let mut chain = ComponentChain::<Messages>::new(100)
        .add_cmp(cmp_redis_subscriber::create(cmp_redis_subscriber::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel.clone(),
        }))
        .add_cmp(cmp_http_server::new(cmp_http_server::Config {
            port: config.http_server_port,
        }))
        .add_cmp(cmp_redis_publisher::create(cmp_redis_publisher::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel,
        }));

    chain.spawn().await;
}

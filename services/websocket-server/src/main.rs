use rsiot_redis_publisher::cmp_redis_publisher;
use rsiot_redis_subscriber::cmp_redis_subscriber;
use rsiot_websocket_server::{cmp_websocket_server, ComponentChain, IMessage};
use tokio::main;
use tracing::Level;

use logging::configure_logging;
use messages::Messages;

#[main]
async fn main() {
    let config = env_vars::load_config().expect("Settings not loaded");

    configure_logging("websocket-server", &config.loki_url, Level::DEBUG)
        .await
        .expect("Error in logger initialization");

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_redis_subscriber::create(cmp_redis_subscriber::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel.clone(),
        }))
        .add_cmp(cmp_websocket_server::new(cmp_websocket_server::Config {
            port: config.api_ws_port,
            fn_send_to_client: |msg: Messages| msg.to_json().ok(),
            fn_recv_from_client: |data: &str| Messages::from_json(data).ok(),
        }))
        .add_cmp(cmp_redis_publisher::create(cmp_redis_publisher::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel,
        }));

    chain.spawn().await;
}

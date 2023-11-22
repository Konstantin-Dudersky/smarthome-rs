mod config_fn_recv;
mod config_fn_send;
mod messages_from_ws;

use rsiot::{cmp_logger, cmp_redis_publisher, cmp_websocket_client, component::ComponentChain};
use tokio::main;
use tracing::Level;

use env_vars::load_config;
use logging::configure_logging;

use crate::{config_fn_recv::fn_recv, config_fn_send::fn_send};

#[main]
async fn main() {
    let config = load_config().expect("Файл настроек не загружен");

    configure_logging("deconz-ws", &config.loki_url, Level::TRACE)
        .await
        .expect("Логгирование не настроено");

    let mut chain = ComponentChain::init(100)
        .start_cmp(cmp_websocket_client::new(cmp_websocket_client::Config {
            url: config.deconz_hub_ws(),
            fn_send,
            fn_recv,
        }))
        .then_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
        }))
        .end_cmp(cmp_redis_publisher::create(cmp_redis_publisher::Config {
            url: config.redis_url(),
            redis_channel: "smarthome".into(),
        }));

    chain.spawn().await
}

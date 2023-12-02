mod config_fn_input;
mod config_fn_output;
mod messages_from_ws;

use rsiot::{
    cmp_redis_publisher, cmp_websocket_client,
    component::{cmp_logger, ComponentChain},
};
use tokio::main;
use tracing::Level;

use env_vars::{load_config, Config};
use logging::configure_logging;

use crate::{config_fn_input::fn_input, config_fn_output::fn_output};

#[main]
async fn main() {
    let config = load_config::<Config>().expect("Файл настроек не загружен");

    configure_logging("deconz-ws", &config.loki_url(), Level::INFO)
        .await
        .expect("Логгирование не настроено");

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_websocket_client::new(cmp_websocket_client::Config {
            url: config.deconz_hub_ws(),
            fn_send: fn_input,
            fn_recv: fn_output,
        }))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::TRACE,
            header: "".into(),
        }))
        .add_cmp(cmp_redis_publisher::create(cmp_redis_publisher::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel,
        }));

    chain.spawn().await
}

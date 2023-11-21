mod config_fn_recv;
mod config_fn_send;
mod messages_from_ws;

use rsiot::{
    cmp_logger, cmp_redis_publisher, cmp_websocket_client,
    component::ComponentChain,
};
use tokio::main;
use tracing::{level_filters::LevelFilter, Level};
use url::Url;

use crate::{config_fn_recv::fn_recv, config_fn_send::fn_send};

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mut chain = ComponentChain::init(100)
        .start_cmp(cmp_websocket_client::create(cmp_websocket_client::Config {
            url: Url::parse("ws://target:8012").unwrap(),
            fn_send,
            fn_recv,
        }))
        .then_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
        }))
        .end_cmp(cmp_redis_publisher::create(cmp_redis_publisher::Config {
            url: Url::parse("redis://target:6379").unwrap(),
            redis_channel: "smarthome".into(),
        }));

    chain.spawn().await
}

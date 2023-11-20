mod messages_from_ws;

use serde::{Deserialize, Serialize};
use serde_json::from_str as deserizlize;
use tokio::main;
use tracing::{info, warn, Level};
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_logger;
use rsiot_messages_core::IMessage;
use rsiot_websocket_client::cmp_websocket_client;

use crate::messages_from_ws::WsMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Message {}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut chain = ComponentChain::init(100)
        .start_cmp(cmp_websocket_client::create(cmp_websocket_client::Config {
            url: Url::parse("ws://target:8012").unwrap(),
            fn_send: fn_send,
            fn_recv: fn_recv,
        }))
        .end_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
        }));

    chain.spawn().await
}

fn fn_send(msg: Message) -> Option<String> {
    None
}

fn fn_recv(data: String) -> Vec<Message> {
    let json = deserizlize::<WsMessage>(&data);
    if let Ok(ws_msg) = json {
        info!("New message: {:?}", ws_msg);
        return vec![];
    }
    let err = format!("Unknown message: {:?}", data);
    warn!(err);
    return vec![];
}

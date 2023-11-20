mod messages_from_ws;

use serde::{Deserialize, Serialize};
use serde_json::from_str as deserizlize;
use tokio::main;
use tracing::{debug, info, level_filters::LevelFilter, warn, Level};
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_logger;
use rsiot_messages_core::IMessage;
use rsiot_websocket_client::cmp_websocket_client;

use crate::messages_from_ws::WsMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    ButtonEvent(u16),
    RoomTemperature(f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

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
        debug!("New message: {:?}", ws_msg);
        match ws_msg.uniqueid.as_str() {
            // Кнопка
            "00:15:8d:00:02:5f:1e:77-01-0006" => {
                match ws_msg.state {
                    messages_from_ws::State::ZHASwitch(val) => {
                        let msg = Message::ButtonEvent(val.buttonevent);
                        return vec![msg];
                    }
                    _ => (),
                };
            }
            // Датчик температуры в комнате
            "00:15:8d:00:03:f0:44:0d-01-0402" => match ws_msg.state {
                messages_from_ws::State::ZHATemperature(val) => {
                    let val = val.temperature as f64 / 100.0;
                    let msg = Message::RoomTemperature(val);
                    return vec![msg];
                }
                _ => (),
            },
            // Датчик влажности в комнате
            "00:15:8d:00:03:f0:44:0d-01-0405" => {}
            // Датчик давления в комнате
            "00:15:8d:00:03:f0:44:0d-01-0403" => {}
            _ => (),
        }

        return vec![];
    }
    let err = format!("Unknown message: {:?}", data);
    warn!(err);
    return vec![];
}

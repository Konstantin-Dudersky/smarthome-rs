//! Преобразование полученных данных от Deconz в сообщения

use serde_json::from_str as deserialize;
use tracing::{debug, warn};

use messages::{types::SingleValue, Messages};

use crate::messages_from_ws::{State, WsMessage};

pub fn fn_recv(data: String) -> Vec<Messages> {
    let json = deserialize::<WsMessage>(&data);
    if let Ok(ws_msg) = json {
        debug!("New message: {:?}", ws_msg);
        match ws_msg.uniqueid.as_str() {
            // Кнопка
            "00:15:8d:00:02:5f:1e:77-01-0006" => {
                match ws_msg.state {
                    State::ZHASwitch(val) => {
                        let msg = Messages::ButtonEvent(SingleValue::new(
                            val.buttonevent,
                            None,
                        ));
                        return vec![msg];
                    }
                    _ => (),
                };
            }
            // Датчик температуры в комнате
            "00:15:8d:00:03:f0:44:0d-01-0402" => match ws_msg.state {
                State::ZHATemperature(val) => {
                    let val = val.temperature as f64 / 100.0;
                    let msg =
                        Messages::RoomTemperature(SingleValue::new(val, None));
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

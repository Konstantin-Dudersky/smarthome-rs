//! Преобразование полученных данных от Deconz в сообщения

use rsiot::message::msg_types;
use serde_json::from_str as deserialize;
use tracing::{debug, warn};

use messages::Messages;

use crate::messages_from_ws::{State, WsMessage};

pub fn fn_output(data: String) -> Vec<Messages> {
    let json = deserialize::<WsMessage>(&data);
    if let Ok(ws_msg) = json {
        debug!("New message: {:?}", ws_msg);
        match ws_msg.uniqueid.as_str() {
            // Кнопка
            "00:15:8d:00:02:5f:1e:77-01-0006" => {
                match ws_msg.state {
                    State::ZHASwitch(state) => {
                        let value = state.buttonevent;
                        let ts = state.lastupdated;
                        let msg = Messages::ButtonEvent(msg_types::Value::new_with_ts(value, ts));
                        return vec![msg];
                    }
                    _ => (),
                };
            }
            // Датчик температуры в комнате
            "00:15:8d:00:03:f0:44:0d-01-0402" => match ws_msg.state {
                State::ZHATemperature(state) => {
                    let temperature = state.temperature as f64 / 100.0;
                    let ts = state.lastupdated;
                    let msg =
                        Messages::RoomTemperature(msg_types::Value::new_with_ts(temperature, ts));
                    return vec![msg];
                }
                _ => (),
            },
            // Датчик влажности в комнате
            "00:15:8d:00:03:f0:44:0d-01-0405" => match ws_msg.state {
                State::ZHAHumidity(state) => {
                    let humidity = state.humidity as f64 / 100.0;
                    let ts = state.lastupdated;
                    let msg = Messages::RoomHumidity(msg_types::Value::new_with_ts(humidity, ts));
                    return vec![msg];
                }
                _ => (),
            },
            // Датчик давления в комнате
            "00:15:8d:00:03:f0:44:0d-01-0403" => match ws_msg.state {
                State::ZHAPressure(state) => {
                    let pressure = (state.pressure as f64) * 1000.0;
                    let ts = state.lastupdated;
                    let msg = Messages::RoomPressure(msg_types::Value::new_with_ts(pressure, ts));
                    return vec![msg];
                }
                _ => (),
            },
            // Датчик температуры в ванной
            "00:15:8d:00:03:cd:1c:97-01-0402" => match ws_msg.state {
                State::ZHATemperature(state) => {
                    let temperature = state.temperature as f64 / 100.0;
                    let ts = state.lastupdated;
                    let msg =
                        Messages::BathTemperature(msg_types::Value::new_with_ts(temperature, ts));
                    return vec![msg];
                }
                _ => (),
            },
            // Датчик влажности в ванной
            "00:15:8d:00:03:cd:1c:97-01-0405" => match ws_msg.state {
                State::ZHAHumidity(state) => {
                    let humidity = state.humidity as f64 / 100.0;
                    let ts = state.lastupdated;
                    let msg = Messages::BathHumidity(msg_types::Value::new_with_ts(humidity, ts));
                    return vec![msg];
                }
                _ => (),
            },
            // Датчик давления в ванной
            "00:15:8d:00:03:cd:1c:97-01-0403" => match ws_msg.state {
                State::ZHAPressure(state) => {
                    let pressure = (state.pressure as f64) * 1000.0;
                    let ts = state.lastupdated;
                    let msg = Messages::BathPressure(msg_types::Value::new_with_ts(pressure, ts));
                    return vec![msg];
                }
                _ => (),
            },
            _ => (),
        }

        return vec![];
    }
    let err = format!("Unknown message: {:?}", data);
    warn!(err);
    return vec![];
}

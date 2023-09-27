use tracing::warn;

use messages::{types, Messages as RMessage};

use crate::messages_from_ws::{Message as WsMessage, State};

pub fn process_message(ws_msg: WsMessage) -> Option<RMessage> {
    const UUID_OPEN_CLOSE: &str = "00:15:8d:00:03:21:44:8c-01-0006";
    const UUID_ROOM_TEMP: &str = "00:15:8d:00:03:f0:44:0d-01-0402";
    const UUID_ROOM_HUMIDITY: &str = "00:15:8d:00:03:f0:44:0d-01-0405";
    const UUID_ROOM_PRESSURE: &str = "00:15:8d:00:03:f0:44:0d-01-0403";

    match ws_msg.uniqueid.as_str() {
        UUID_OPEN_CLOSE => return open_close(ws_msg),
        UUID_ROOM_TEMP => return room_temperature(ws_msg),
        UUID_ROOM_HUMIDITY => return room_humidity(ws_msg),
        UUID_ROOM_PRESSURE => return room_pressure(ws_msg),
        _ => warn!("Неизвестный UUID: {}", ws_msg.uniqueid),
    }
    None
}

fn open_close(ws_msg: WsMessage) -> Option<RMessage> {
    let (value, ts) = match ws_msg.state {
        State::ZHAOpenCloseState(state) => (state.open, state.lastupdated),
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", ws_msg);
            return None;
        }
    };
    let msg =
        RMessage::OpenCloseSensor(types::SingleValue::new(value, Some(ts)));
    Some(msg)
}

fn room_temperature(ws_msg: WsMessage) -> Option<RMessage> {
    let value = match ws_msg.state {
        State::ZHATemperature(state) => state.temperature,
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", ws_msg);
            return None;
        }
    };
    let value = value as f64 / 100.0;
    let msg = RMessage::RoomTemperature(types::SingleValue::new(value, None));
    Some(msg)
}

fn room_humidity(ws_msg: WsMessage) -> Option<RMessage> {
    let value = match ws_msg.state {
        State::ZHAHumidity(state) => state.humidity,
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", ws_msg);
            return None;
        }
    };
    let value = value as f64 / 100.0;
    let msg = RMessage::RoomHumidity(types::SingleValue::new(value, None));
    Some(msg)
}

fn room_pressure(ws_msg: WsMessage) -> Option<RMessage> {
    let value = match ws_msg.state {
        State::ZHAPressure(state) => state.pressure,
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", ws_msg);
            return None;
        }
    };
    let value = value as f64;
    let msg = RMessage::RoomPressure(types::SingleValue::new(value, None));
    Some(msg)
}

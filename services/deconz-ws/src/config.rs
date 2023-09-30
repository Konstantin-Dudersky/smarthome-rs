use tracing::warn;

use messages::{types, Messages as RMessage};

use crate::messages_from_ws::{Sensor, State, WsMessage};

pub fn process_ws_message(ws_msg: WsMessage) -> Option<RMessage> {
    process_msg(&ws_msg.uniqueid, &ws_msg.state)
}

pub fn process_api_message(sensor: Sensor) -> Option<RMessage> {
    process_msg(&sensor.uniqueid, &sensor.state)
}

fn process_msg(uniqueid: &str, state: &State) -> Option<RMessage> {
    const UUID_OPEN_CLOSE: &str = "00:15:8d:00:03:21:44:8c-01-0006";
    const UUID_ROOM_TEMP: &str = "00:15:8d:00:03:f0:44:0d-01-0402";
    const UUID_ROOM_HUMIDITY: &str = "00:15:8d:00:03:f0:44:0d-01-0405";
    const UUID_ROOM_PRESSURE: &str = "00:15:8d:00:03:f0:44:0d-01-0403";

    match uniqueid {
        UUID_OPEN_CLOSE => return open_close(state),
        UUID_ROOM_TEMP => return room_temperature(state),
        UUID_ROOM_HUMIDITY => return room_humidity(state),
        UUID_ROOM_PRESSURE => return room_pressure(state),
        _ => warn!("Неизвестный UUID: {}", uniqueid),
    }
    None
}

fn open_close(state: &State) -> Option<RMessage> {
    let (value, ts) = match state {
        State::ZHAOpenClose(state) => (state.open, state.lastupdated),
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", state);
            return None;
        }
    };
    let msg =
        RMessage::OpenCloseSensor(types::SingleValue::new(value, Some(ts)));
    Some(msg)
}

fn room_temperature(state: &State) -> Option<RMessage> {
    let value = match state {
        State::ZHATemperature(state) => state.temperature,
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", state);
            return None;
        }
    };
    let value = value as f64 / 100.0;
    let msg = RMessage::RoomTemperature(types::SingleValue::new(value, None));
    Some(msg)
}

fn room_humidity(state: &State) -> Option<RMessage> {
    let value = match state {
        State::ZHAHumidity(state) => state.humidity,
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", state);
            return None;
        }
    };
    let value = value as f64 / 100.0;
    let msg = RMessage::RoomHumidity(types::SingleValue::new(value, None));
    Some(msg)
}

fn room_pressure(state: &State) -> Option<RMessage> {
    let value = match state {
        State::ZHAPressure(state) => state.pressure,
        _ => {
            warn!("Невозможно извлечь состояние из сообщения: {:?}", state);
            return None;
        }
    };
    let value = value as f64;
    let msg = RMessage::RoomPressure(types::SingleValue::new(value, None));
    Some(msg)
}

use crate::messages_from_ws::Message as WsMessage;
use messages::{types, Messages as RMessage};

pub fn process_message(ws_msg: WsMessage) -> Option<RMessage> {
    let uuid_open_close = "00:15:8d:00:03:21:44:8c-01-0006";

    if ws_msg.uniqueid == uuid_open_close {
        let msg =
            RMessage::OpenCloseSensor(types::SingleValue::new(false, None));
        return Some(msg);
    }

    match ws_msg.state {
        crate::messages_from_ws::State::ZHAHumidity(_) => None,
        crate::messages_from_ws::State::ZHALightLevel(_) => None,
        crate::messages_from_ws::State::ZHAOpenCloseState(_) => None,
        crate::messages_from_ws::State::ZHAPresence(_) => None,
        crate::messages_from_ws::State::ZHAPressure(_) => None,
        crate::messages_from_ws::State::ZHASwitch(_) => None,
        crate::messages_from_ws::State::ZHATemperature(_) => None,
    }
}

use crate::messages_from_ws::{Message as WsMessage, State};
use messages::{types, Messages as RMessage};

pub fn process_message(ws_msg: WsMessage) -> Option<RMessage> {
    let uuid_open_close = "00:15:8d:00:03:21:44:8c-01-0006";

    if ws_msg.uniqueid == uuid_open_close {
        return open_close(ws_msg);
    }
    None
}

fn open_close(ws_msg: WsMessage) -> Option<RMessage> {
    let value = match ws_msg.state {
        State::ZHAOpenCloseState(state) => state.open,
        _ => return None,
    };
    let msg = RMessage::OpenCloseSensor(types::SingleValue::new(value, None));
    Some(msg)
}

use leptos::*;
use url::Url;

use messages::{IMessage, Messages};
use rsiot::message::msg_types;

#[derive(Copy, Clone, Debug, Default)]
pub struct MsgData {
    pub room_temperature: RwSignal<msg_types::Value<f64>>,
    pub room_humidity: RwSignal<msg_types::Value<f64>>,
    pub room_pressure: RwSignal<msg_types::Value<f64>>,

    pub send_msg: RwSignal<Option<Messages>>,
    pub window_url: RwSignal<Option<Url>>,
    pub motor_state: RwSignal<msg_types::Value<i16>>,
}

pub fn process_ws_message(msg: &str) {
    let global_state = use_context::<MsgData>().expect("no global state");
    let msg = Messages::from_json(msg).unwrap();
    match msg {
        Messages::ButtonEvent(_) => (),
        Messages::OpenCloseSensor(_) => (),
        Messages::RoomHumidity(value) => global_state.room_humidity.set(value),
        Messages::RoomTemperature(value) => global_state.room_temperature.set(value),
        Messages::RoomPressure(value) => global_state.room_pressure.set(value),
        Messages::BathTemperature(_) => (),
        Messages::BathHumidity(_) => (),
        Messages::BathPressure(_) => (),
    };
}

use std::str::FromStr;

use leptos::*;
use url::Url;

use messages::{types, IMessage, Messages};

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub send_msg: RwSignal<Option<Messages>>,
    pub window_url: RwSignal<Url>,
    pub api_url: RwSignal<String>,
    pub room_temperature: RwSignal<types::SingleValue<f64>>,
    pub motor_state: RwSignal<types::SingleValue<i16>>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            send_msg: create_rw_signal(None),
            window_url: create_rw_signal(Url::from_str("http://localhost").unwrap()),
            api_url: create_rw_signal("".to_string()),

            room_temperature: create_rw_signal(types::SingleValue::default()),
            motor_state: create_rw_signal(types::SingleValue::default()),
        }
    }
}

pub fn process_ws_message(msg: &str) {
    let global_state = use_context::<GlobalState>().expect("no global state");
    let msg = Messages::from_json(msg).unwrap();
    match msg {
        Messages::RoomTemperature(value) => global_state.room_temperature.set(value),
        Messages::OpenCloseSensor(_) => todo!(),
        Messages::RoomHumidity(_) => todo!(),
        Messages::RoomPressure(_) => todo!(),
        Messages::ButtonEvent(_) => todo!(),
    };
}

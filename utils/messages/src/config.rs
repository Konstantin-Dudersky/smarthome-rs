use serde::{Deserialize, Serialize};

use rsiot::message::IMessage;

use crate::types;

/// Все сообщения в системе
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Messages {
    OpenCloseSensor(types::SingleValue<bool>),
    /// Датчик температуры / влажности
    RoomTemperature(types::SingleValue<f64>),
    RoomHumidity(types::SingleValue<f64>),
    RoomPressure(types::SingleValue<f64>),
    /// Событие нажатия кнопки
    ButtonEvent(types::SingleValue<u16>),
}

impl IMessage for Messages {}

// test ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let msg1 =
            Messages::RoomTemperature(types::SingleValue::new(10.0, None));
        assert_eq!("MotorState", msg1.key());
    }

    #[test]
    fn ser_deser() {
        let msg1 =
            Messages::RoomTemperature(types::SingleValue::new(10.0, None));

        let ser = msg1.to_json().unwrap();
        let deser = Messages::from_json(&ser).unwrap();

        assert_eq!(msg1, deser);
    }
}

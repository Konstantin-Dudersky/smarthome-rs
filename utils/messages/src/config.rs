use serde::{Deserialize, Serialize};

use rsiot::message::{eav, eav_helpers, msg_types, IMessage};

/// Все сообщения в системе
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Messages {
    OpenCloseSensor(msg_types::Value<bool>),
    /// Датчик температуры / влажности в комнате
    RoomTemperature(msg_types::Value<f64>),
    RoomHumidity(msg_types::Value<f64>),
    RoomPressure(msg_types::Value<f64>),
    /// Датчик температуры / влажности в ванной
    BathTemperature(msg_types::Value<f64>),
    BathHumidity(msg_types::Value<f64>),
    BathPressure(msg_types::Value<f64>),
    /// Событие нажатия кнопки
    ButtonEvent(msg_types::Value<u16>),
}

impl IMessage for Messages {
    fn into_eav(self) -> Vec<eav::EavModel> {
        match self {
            Messages::OpenCloseSensor(_) => vec![],
            Messages::RoomTemperature(val) => eav_helpers::ValueInstant {
                ts: val.ts,
                entity: "RoomTemperature".into(),
                attr: "".into(),
                value: val.value.into(),
            }
            .into(),
            Messages::RoomHumidity(val) => eav_helpers::ValueInstant {
                ts: val.ts,
                entity: "RoomHumidity".into(),
                attr: "".into(),
                value: val.value.into(),
            }
            .into(),
            Messages::RoomPressure(val) => eav_helpers::ValueInstant {
                ts: val.ts,
                entity: "RoomPressure".into(),
                attr: "".into(),
                value: val.value.into(),
            }
            .into(),
            Messages::BathTemperature(val) => eav_helpers::ValueInstant {
                ts: val.ts,
                entity: "BathTemperature".into(),
                attr: "".into(),
                value: val.value.into(),
            }
            .into(),
            Messages::BathHumidity(val) => eav_helpers::ValueInstant {
                ts: val.ts,
                entity: "BathHumidity".into(),
                attr: "".into(),
                value: val.value.into(),
            }
            .into(),
            Messages::BathPressure(val) => eav_helpers::ValueInstant {
                ts: val.ts,
                entity: "BathPressure".into(),
                attr: "".into(),
                value: val.value.into(),
            }
            .into(),
            Messages::ButtonEvent(_) => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let msg1 = Messages::RoomTemperature(msg_types::Value::new(10.0));
        assert_eq!("MotorState", msg1.key());
    }

    #[test]
    fn ser_deser() {
        let msg1 = Messages::RoomTemperature(msg_types::Value::new(10.0));

        let ser = msg1.to_json().unwrap();
        let deser = Messages::from_json(&ser).unwrap();

        assert_eq!(msg1, deser);
    }
}

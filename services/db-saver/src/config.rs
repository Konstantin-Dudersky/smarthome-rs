use db_saver_lib::models::Row;
use messages::Messages;

use rsiot::message::IMessage;

/// Преобразование сообщений из Redis в строки для базы данных
pub fn prepare_msg_from_redis_to_db(msg: Messages) -> Option<Row> {
    let entity = msg.key();
    match &msg {
        // SingleValue<bool>
        Messages::OpenCloseSensor(value) => {
            Some(Row::new(value.ts, &entity, "", value.value as u8 as f64))
        } // SingleValue<f64>
        Messages::RoomTemperature(value)
        | Messages::RoomHumidity(value)
        | Messages::RoomPressure(value) => {
            Some(Row::new(value.ts, &entity, "", value.value as f64))
        }
        Messages::ButtonEvent(value) => {
            Some(Row::new(value.ts, &entity, "", value.value as f64))
        }
    }
}

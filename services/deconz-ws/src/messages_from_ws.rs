use chrono::{DateTime, FixedOffset};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub e: String,
    pub id: String,
    pub r: String,
    pub t: String,
    pub state: State,
    pub uniqueid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum State {
    ZHAHumidity(ZHAHumidity),
    ZHALightLevel(ZHALightLevel),
    ZHAOpenCloseState(ZHAOpenCloseState),
    ZHAPresence(ZHAPresence),
    ZHAPressure(ZHAPressure),
    ZHASwitch(ZHASwitch),
    ZHATemperature(ZHATemperature),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHAHumidity {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub humidity: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHALightLevel {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub dark: bool,
    pub daylight: bool,
    pub lightlevel: u32,
    pub lux: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHAOpenCloseState {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub open: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHAPresence {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub presence: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHAPressure {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub pressure: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHASwitch {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub buttonevent: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZHATemperature {
    #[serde(deserialize_with = "parse_lastupdated")]
    pub lastupdated: DateTime<FixedOffset>,
    pub temperature: u16,
}

/// Метка времени в сообщениях из Deconz приводится в поясе UTC, но часовой
/// пояс не указан. Добавляем букву Z
fn parse_lastupdated<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s + "Z";
    match chrono::DateTime::parse_from_rfc3339(&s) {
        Ok(value) => Ok(value),
        Err(error) => {
            let msg = format!("{error}, value: {s}");
            Err(Error::custom(msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::from_str as deserialize;

    #[test]
    fn test_open_close() {
        let json = vec![
            // ZHAOpenCloseState
            r#"{
                "e": "changed",
                "id": "7",
                "r": "sensors",
                "state": {
                  "lastupdated": "2023-09-23T09:25:38.851",
                  "open": false
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:03:21:44:8c-01-0006"
              }"#,
            // ZHAPresence
            r#"{
                "e": "changed",
                "id": "2",
                "r": "sensors",
                "state": {
                    "lastupdated": "2023-09-23T10:51:46.714",
                    "presence": true
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:07:58:e9:5e-01-0406"
            }"#,
            // ZHALightLevel
            r#"{
                "e": "changed",
                "id": "3",
                "r": "sensors",
                "state": {
                    "dark": false,
                    "daylight": false,
                    "lastupdated": "2023-09-23T11:02:28.851",
                    "lightlevel": 18922,
                    "lux": 78
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:07:58:e9:5e-01-0400"
            }"#,
            // ZHAPressure
            r#"{
                "e": "changed",
                "id": "6",
                "r": "sensors",
                "state": {
                    "lastupdated": "2023-09-23T11:14:39.047",
                    "pressure": 975
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:03:f0:44:0d-01-0403"
            }"#,
            // ZHAHumidity
            r#"{
                "e": "changed",
                "id": "5",
                "r": "sensors",
                "state": {
                    "humidity": 4948,
                    "lastupdated": "2023-09-23T11:14:39.043"
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:03:f0:44:0d-01-0405"
            }"#,
            // ZHATemperature
            r#"{
                "e": "changed",
                "id": "4",
                "r": "sensors",
                "state": {
                    "lastupdated": "2023-09-23T11:14:39.035",
                    "temperature": 2836
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:03:f0:44:0d-01-0402"
            }"#,
            // ZHASwitch
            r#"{
                "e": "changed",
                "id": "8",
                "r": "sensors",
                "state": {
                    "buttonevent": 1002,
                    "lastupdated": "2023-09-23T12:09:00.019"
                },
                "t": "event",
                "uniqueid": "00:15:8d:00:02:5f:1e:77-01-0006"
            }"#,
        ];

        json.iter().for_each(|msg| {
            deserialize::<Message>(&msg).expect("Тест не прошел");
        });
    }
}

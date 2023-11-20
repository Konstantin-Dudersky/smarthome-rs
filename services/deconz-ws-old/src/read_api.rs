use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::messages_from_ws::Sensor;
use crate::types::MyResult;

#[derive(Debug, Deserialize, Serialize)]
struct All {
    groups: HashMap<String, String>,
    sensors: HashMap<String, Sensor>,
}

pub async fn read_state_from_api() -> MyResult<Vec<Sensor>> {
    let resp: Vec<Sensor> = reqwest::get("http://target:8013/api/1234567890")
        .await
        .unwrap()
        .json::<All>()
        .await
        .unwrap()
        .sensors
        .values()
        .cloned()
        .collect();
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn read_api() {
        read_state_from_api().await.unwrap();
    }
}

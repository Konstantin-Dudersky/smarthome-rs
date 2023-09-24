//! Структура для тестирования отправки и получения значений

use serde::{Deserialize, Serialize};

use crate::GetKey;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TestRedisValue<T> {
    pub key: String,
    pub value: T,
}

impl<T> GetKey for TestRedisValue<T> {
    fn key(&self) -> String {
        self.key.clone()
    }
}

impl<T> TestRedisValue<T> {
    pub fn new(key: &str, value: T) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}

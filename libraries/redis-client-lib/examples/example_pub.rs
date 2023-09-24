//! Пример реализации публикации сообщений, используя асинхронный клиент

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio::main;
use url::Url;

use redis_client_lib::{GetKey, RedisPubAsync};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum Tags {
    Field1(SimpleValue<u32>),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct SimpleValue<T> {
    value: T,
}

impl GetKey for Tags {
    fn key(&self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[main]
async fn main() {
    let url = Url::from_str("redis://127.0.0.1").expect("");
    let mut hash = RedisPubAsync::new(&url, "test_hash")
        .await
        .expect("Соединение не создано");

    let value = Tags::Field1(SimpleValue { value: 10 });

    hash.set(value.clone()).await.unwrap();
    let key = value.key();
    let read_field: Tags = hash.get(&key).await.unwrap();

    assert_eq!(read_field, value);
}

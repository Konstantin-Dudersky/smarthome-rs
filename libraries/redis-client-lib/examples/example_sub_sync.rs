//! Пример реализации подписки на новые сообщения

use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

use tokio::main;

use redis_client_lib::{
    start_redis_subscription, RedisPubAsync, TestRedisValue,
};
use url::Url;

#[main]
async fn main() {
    let url = Url::from_str("redis://127.0.0.1").expect("");
    let channel = "test_sub";
    let msg_content =
        TestRedisValue::new("test_sub", "test sub value".to_string());

    let (tx, rx) = mpsc::channel::<TestRedisValue<String>>();

    // запускаем поток с подпиской
    let url_clone = url.clone();
    thread::spawn(move || {
        start_redis_subscription(&url_clone, channel, &tx).unwrap();
    });

    // отправляем сообщение
    let mut redis_hash = RedisPubAsync::new(&url, channel).await.unwrap();
    redis_hash.set(msg_content.clone()).await.unwrap();

    // проверяем, что сообщение пришло
    for msg in rx {
        assert_eq!(msg_content, msg);
        break;
    }
}

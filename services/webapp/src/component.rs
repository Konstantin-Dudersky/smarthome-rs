//! Пример "пустого" компонента. Для тестирования

use std::time::Duration;

use gloo::timers::future::sleep;
use rsiot::message::{eav, eav_helpers, msg_types::Value, IMessage};
use rsiot_component_singlethread::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput,
};
use tokio::task::spawn_local;
use tracing::info;

use crate::message::Messages;

async fn fn_process(
    _input: ComponentInput<Messages>,
    output: ComponentOutput<Messages>,
    _config: Config,
    _cache: Cache<Messages>,
) -> Result<(), ComponentError> {
    let mut counter = 0;
    loop {
        info!("Component counter: {}", counter);
        counter += 1;
        let msg = Messages::BathPressure(Value::new(counter as f64));
        output.send(msg).await;
        sleep(Duration::from_secs(2)).await;
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {}

pub fn new(config: Config) -> Box<Component<Messages, Config>> {
    Box::new(Component::new(config, fn_process))
}

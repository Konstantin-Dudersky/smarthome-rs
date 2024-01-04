mod config_fn_input;
mod config_fn_output;
mod messages_from_ws;

use std::time::Duration;

use rsiot::{
    component_core::ComponentCollection,
    components::{cmp_logger, cmp_redis_client, cmp_websocket_client},
    // logging::configure_logging,
};
use tokio::{main, time::sleep};
use tracing::{error, Level};

use env_vars::{load_config, Config};
use messages::MessageChannel;

use crate::{config_fn_input::fn_input, config_fn_output::fn_output};

#[main]
async fn main() {
    let config = load_config::<Config>().expect("Файл настроек не загружен");

    configure_logging(&config.loki_url())
        .await
        .expect("Логгирование не настроено");

    let redis_config = cmp_redis_client::Config {
        url: config.redis_url(),
        subscription_channel: MessageChannel::Output,
        fn_input: |_| vec![MessageChannel::Output],
    };

    let mut chain = ComponentCollection::new(
        100,
        vec![
            cmp_websocket_client::new(cmp_websocket_client::Config {
                url: config.deconz_hub_ws(),
                fn_send: fn_input,
                fn_recv: fn_output,
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::TRACE,
                header: "".into(),
            }),
            cmp_redis_client::new(redis_config),
        ],
    );

    let result = chain.spawn().await;
    error!("{:?}", result);
    sleep(Duration::from_secs(2)).await;
}

// TODO - импортировать из rsiot, удалить

use std::env;
use tracing::info;
use url::Url;

// #[cfg(any(target_arch = "x86_64"))]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub async fn configure_logging(loki_url: &Url) -> Result<(), tracing_loki::Error> {
    use tokio::spawn;
    use tracing_subscriber::{prelude::*, EnvFilter};

    let service = env::args().collect::<Vec<String>>()[0].clone();

    let (layer_loki, task) = tracing_loki::builder()
        .label("service", service.clone())?
        .build_url(loki_url.clone())?;

    // архивируем в консоль только в дебаг режиме
    let layer_stdout = match cfg!(debug_assertions) {
        true => Some(tracing_subscriber::fmt::Layer::new().pretty()),
        false => None,
    };

    tracing_subscriber::registry()
        .with(layer_loki)
        .with(layer_stdout)
        .with(EnvFilter::from_default_env())
        .init();

    spawn(task);

    info!("service {} started", service);
    Ok(())
}

use std::time::Duration;

use tokio::{main, time::sleep};

use env_vars::load_config;
use logging::configure_logging;
use tracing::{debug, error, info, trace, warn, Level};

#[main]
async fn main() {
    let config = load_config().expect("Файл настроек не загружен");

    configure_logging("deconz-ws1", &config.loki_url, Level::INFO)
        .await
        .expect("Логгирование не настроено");

    trace!("message");
    debug!("message");
    info!("message");
    warn!("message");
    error!("message");

    sleep(Duration::from_secs(2)).await;
}

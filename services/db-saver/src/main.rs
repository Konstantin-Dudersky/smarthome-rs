use tokio::main;
use tracing::Level;

use env_vars::load_config;
use logging::configure_logging;
use rsiot_redis_subscriber::cmp_redis_subscriber;
use rsiot_timescaledb_storing::{cmp_timescaledb_storing, ComponentChain};

mod config;

#[main]
async fn main() {
    let config = load_config().expect("Settings not loaded");

    configure_logging("db-saver", &config.loki_url, Level::INFO)
        .await
        .expect("Error in logger initialization");

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_redis_subscriber::create(cmp_redis_subscriber::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel.clone(),
        }))
        .add_cmp(cmp_timescaledb_storing::new(
            cmp_timescaledb_storing::Config {
                fn_process: config::prepare_msg_from_redis_to_db,
                connection_string: config.db_data_url(),
            },
        ));

    chain.spawn().await;
}

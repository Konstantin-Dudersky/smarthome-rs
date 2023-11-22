use tokio::main;
use tracing::Level;

use env_vars::load_config;
use logging::configure_logging;
use rsiot::{cmp_redis_subscriber, cmp_timescaledb_storing, component::ComponentChain};

mod config;

#[main]
async fn main() {
    let config = load_config().expect("Settings not loaded");

    configure_logging("db-saver", &config.loki_url, Level::INFO)
        .await
        .expect("Error in logger initialization");

    let mut chain = ComponentChain::init(100)
        .start_cmp(cmp_redis_subscriber::create(cmp_redis_subscriber::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel.clone(),
        }))
        .end_cmp(cmp_timescaledb_storing::new(
            cmp_timescaledb_storing::Config {
                fn_process: config::prepare_msg_from_redis_to_db,
                connection_string: config.db_data_url(),
            },
        ));

    chain.spawn().await;
}

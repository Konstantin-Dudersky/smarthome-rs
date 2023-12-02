use tokio::main;
use tracing::Level;

use env_vars::{load_config, Config};
use logging::configure_logging;
use messages::Messages;
use rsiot::{cmp_redis_subscriber, cmp_timescaledb_storing, component::ComponentChain};

#[main]
async fn main() {
    let config = load_config::<Config>().expect("Settings not loaded");

    configure_logging("db-saver", &config.loki_url(), Level::INFO)
        .await
        .expect("Error in logger initialization");

    let mut chain = ComponentChain::<Messages>::new(100)
        .add_cmp(cmp_redis_subscriber::create(cmp_redis_subscriber::Config {
            url: config.redis_url(),
            redis_channel: config.redis_channel.clone(),
        }))
        .add_cmp(cmp_timescaledb_storing::new(
            cmp_timescaledb_storing::Config {
                connection_string: config.db_data_url(),
            },
        ));

    chain.spawn().await;
}

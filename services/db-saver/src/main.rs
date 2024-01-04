use rsiot::{
    component_core::ComponentCollection,
    components::{cmp_redis_client, cmp_timescaledb_storing},
};
use tokio::{
    main,
    time::{sleep, Duration},
};
use tracing::{error, Level};

use env_vars::{load_config, Config};
use logging::configure_logging;
use messages::{MessageChannel, Messages};

#[main]
async fn main() {
    let config = load_config::<Config>().expect("Settings not loaded");

    configure_logging("db-saver", &config.loki_url(), Level::INFO)
        .await
        .expect("Error in logger initialization");

    let redis_config = cmp_redis_client::Config {
        url: config.redis_url(),
        subscription_channel: MessageChannel::Output,
        fn_input: |_| vec![MessageChannel::Output],
    };

    let timescaledb_config = cmp_timescaledb_storing::Config {
        connection_string: config.db_data_url(),
    };

    let mut chain = ComponentCollection::<Messages>::new(
        100,
        vec![
            cmp_redis_client::new(redis_config),
            cmp_timescaledb_storing::new(timescaledb_config),
        ],
    );

    let result = chain.spawn().await;
    error!("{:?}", result);
    sleep(Duration::from_secs(2)).await;
}

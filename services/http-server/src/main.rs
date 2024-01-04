use rsiot::{
    component_core::ComponentCollection,
    components::{cmp_http_server, cmp_redis_client},
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
    let config = load_config::<Config>().expect("Setting not loaded");

    configure_logging("api", &config.loki_url(), Level::INFO)
        .await
        .expect("Error in logger initialization");

    let redis_config = cmp_redis_client::Config {
        url: config.redis_url(),
        subscription_channel: MessageChannel::Output,
        fn_input: |_| vec![MessageChannel::Output],
    };

    let mut chain = ComponentCollection::<Messages>::new(
        100,
        vec![
            cmp_redis_client::new(redis_config),
            cmp_http_server::new(cmp_http_server::Config {
                port: config.http_server_port,
            }),
        ],
    );

    let result = chain.spawn().await;
    error!("{:?}", result);
    sleep(Duration::from_secs(2)).await;
}

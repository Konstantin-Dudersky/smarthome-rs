use rsiot::{
    component_core::ComponentCollection,
    components::{cmp_redis_client, cmp_websocket_server},
    message::IMessage,
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

    configure_logging("websocket-server", &config.loki_url(), Level::DEBUG)
        .await
        .expect("Error in logger initialization");

    let redis_config = cmp_redis_client::Config {
        url: config.redis_url(),
        subscription_channel: MessageChannel::Output,
        fn_input: |_| vec![MessageChannel::Output],
    };

    let ws_server_config = cmp_websocket_server::Config {
        port: config.websocket_server_port,
        fn_input: |msg: &Messages| msg.to_json().ok(),
        fn_output: |data: &str| Messages::from_json(data).ok(),
    };

    let mut chain = ComponentCollection::new(
        100,
        vec![
            cmp_redis_client::new(redis_config),
            cmp_websocket_server::new(ws_server_config),
        ],
    );
    let result = chain.spawn().await;
    error!("{:?}", result);
    sleep(Duration::from_secs(2)).await;
}

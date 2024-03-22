use std::time::Duration;

use gloo::timers::future::sleep;
use leptos::*;
use rsiot::logging::configure_logging;
use rsiot_component_singlethread::ComponentCollection;
use tokio::sync::broadcast;
use tracing::{debug, info};

use webapp_ui::{
    app::App, cmp_websocket_client_wasm::cmp_websocket_client_wasm, component, message::Messages,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    configure_logging("trace");

    let (tx, mut rx) = broadcast::channel::<String>(100);

    let local = tokio::task::LocalSet::new();

    let mut cmps =
        ComponentCollection::<Messages>::new(100, vec![component::new(component::Config {})]);
    local.spawn_local(async move { cmps.spawn().await.unwrap() });

    local.spawn_local(cmp_websocket_client_wasm("ws://target:8081", process_msg));

    let send = tx.clone();
    local.spawn_local(async move {
        tokio::task::spawn_local(async move {
            loop {
                sleep(Duration::from_millis(2000)).await;
                send.send("123".into()).unwrap();
            }
        });
    });

    local.spawn_local(async move {
        loop {
            while let Ok(msg) = rx.recv().await {
                info!("{}", msg);
            }
        }
    });

    spawn_local(async { local.await });

    mount_to_body(|| {
        view! { <App/> }
    })
}

fn process_msg(input: &str) {
    debug!(input);
}

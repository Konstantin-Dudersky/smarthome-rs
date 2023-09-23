use std::time::Duration;

use futures_util::{stream::SplitStream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str as deserialize;
use tokio::{
    main,
    net::TcpStream,
    spawn,
    sync::mpsc::{self, Receiver, Sender},
    time::sleep,
    try_join,
};

use env_vars::load_config;
use logging::configure_logging;
use redis_client_lib::RedisPubAsync;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_util::sync::CancellationToken;
use tracing::{info, trace, warn, Level};

use deconz_ws::{
    async_task_utils::{cancellable_task, flatten_task_result},
    messages_from_ws::Message,
    process_message, Errors, MyResult,
};
use url::Url;

#[main]
async fn main() {
    let config = load_config().expect("Файл настроек не загружен");

    configure_logging("deconz-ws", &config.loki_url, Level::TRACE)
        .await
        .expect("Логгирование не настроено");

    loop {
        let cancel = CancellationToken::new();
        let result = task_main(
            cancel.clone(),
            config.deconz_hub_host.clone(),
            config.deconz_hub_port_ws,
            config.redis_url.clone(),
            config.redis_channel.clone(),
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => warn!("{:?}", err),
        }
        cancel.cancel();
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...");
    }
}

async fn task_main(
    cancel: CancellationToken,
    deconz_hub_host: String,
    deconz_hub_port_ws: u16,
    redis_url: Url,
    redis_channel: String,
) -> MyResult<()> {
    let addr = format!("ws://{deconz_hub_host}:{deconz_hub_port_ws}");
    let (ws_stream, _) = match connect_async(addr).await {
        Ok(val) => val,
        Err(err) => return Err(Errors::CannotConnect(err)),
    };
    let (_, read) = ws_stream.split();

    let (tx, rx) = mpsc::channel::<Message>(128);

    // запускаем поток чтения сообщений из WS
    let future = task_ws_read(read, tx);
    let task_ws_read_handle = spawn(cancellable_task(future, cancel.clone()));

    // запускаем поток отправки сообщений в Redis
    let future = task_redis_pub(rx, redis_url, redis_channel, process_message);
    let task_redis_pub_handle = spawn(cancellable_task(future, cancel.clone()));

    match try_join!(
        flatten_task_result(task_ws_read_handle, Errors::TokioTaskHandleError),
        flatten_task_result(
            task_redis_pub_handle,
            Errors::TokioTaskHandleError
        )
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

/// Задача чтения сообщений из WS
async fn task_ws_read<WM>(
    mut read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    tx: Sender<WM>,
) -> MyResult<()>
where
    WM: std::fmt::Debug + DeserializeOwned,
{
    while let Some(msg) = read_stream.next().await {
        let msg = match msg {
            Ok(val) => val.to_string(),
            Err(err) => return Err(Errors::ReadStream(err)),
        };
        let msg = parse_message(&msg);
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        let send_result = tx.send(msg).await;
        match send_result {
            Ok(_) => (),
            Err(err) => {
                let err = Errors::SendToChannel(err.to_string());
                return Err(err);
            }
        };
    }
    Ok(())
}

async fn task_redis_pub<WM, RM>(
    mut rx: Receiver<WM>,
    redis_url: Url,
    redis_channel: String,
    process_msg: fn(WM) -> Option<RM>,
) -> MyResult<()>
where
    WM: Serialize + Send,
    RM: Serialize + Send,
{
    let mut redis = RedisPubAsync::new(&redis_url, &redis_channel).await?;
    while let Some(msg) = rx.recv().await {
        let msg = process_msg(msg);
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        redis.set("test", msg).await?;
    }
    Ok(())
}

fn parse_message<WM>(msg: &str) -> Option<WM>
where
    WM: std::fmt::Debug + DeserializeOwned,
{
    match deserialize::<WM>(&msg) {
        Ok(value) => {
            trace!("Новое сообщение от Deconz {:?}", value);
            Some(value)
        }
        Err(_) => {
            warn!("Неизвестное сообщение от Deconz: {:?}", msg);
            None
        }
    }
}

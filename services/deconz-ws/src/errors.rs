use redis_client_lib::Errors as RedisError;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

#[derive(Debug)]
pub enum Errors {
    CannotConnect(TungsteniteError),
    TokioTaskHandleError,
    SendToChannel(String),
    ReadStream(TungsteniteError),
    RedisError(RedisError),
}

impl From<RedisError> for Errors {
    fn from(value: RedisError) -> Self {
        Self::RedisError(value)
    }
}

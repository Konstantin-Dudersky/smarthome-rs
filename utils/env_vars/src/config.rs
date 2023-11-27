use serde::{Deserialize, Serialize};
use url::Url;

use rsiot_env_vars::IEnvVars;

/// Настройки системы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,

    pub deconz_hub_api_key: String,
    pub deconz_hub_host: String,
    pub deconz_hub_port_api: u16,
    pub deconz_hub_port_vnc: u16,
    pub deconz_hub_port_ws: u16,
    pub deconz_hub_vnc_password: String,

    pub http_server_port: u16,

    pub grafana_port: u16,

    pub loki_port: u16,
    pub loki_host: String,

    pub portainer_port: u16,

    /// Название канала Pub/Sub, а также хеш-таблицы
    pub redis_channel: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_port_ui: u16,

    pub webapp_port: u16,

    /// Порт websocket-server
    pub websocket_server_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_host: "localhost".into(),
            db_port: 5432,
            db_user: "postgres".into(),
            db_password: "postgres".into(),

            deconz_hub_api_key: "api_key".into(),
            deconz_hub_host: "localhost".into(),
            deconz_hub_port_api: 8010,
            deconz_hub_port_vnc: 8011,
            deconz_hub_port_ws: 8012,
            deconz_hub_vnc_password: "vnc_password".into(),

            http_server_port: 3001,

            grafana_port: 3000,

            loki_port: 3100,
            loki_host: "loki".into(),

            portainer_port: 8001,

            redis_channel: Default::default(),
            redis_host: "target".into(),
            redis_port: 6379,
            redis_port_ui: 8013,

            webapp_port: 8090,

            websocket_server_port: 8081,
        }
    }
}

impl Config {
    /// Подключение к БД с данными
    pub fn db_data_url(&self) -> Url {
        let url = format!(
            "postgres://{}:{}@{}:{}/db_data",
            self.db_user, self.db_password, self.db_host, self.db_port
        );
        Url::parse(&url).expect("Неправильно заданный адрес БД")
    }

    /// URL websocket - сервера
    pub fn deconz_hub_ws(&self) -> Url {
        let url = format!("ws://{}:{}", self.deconz_hub_host, self.deconz_hub_port_ws);
        Url::parse(&url).expect("Неправильно заданный адрес deconz ws сервера")
    }

    pub fn loki_url(&self) -> Url {
        let url = format!("http://{}:{}", self.loki_host, self.loki_port);
        Url::parse(&url).expect("Неправильно заданный адрес loki сервера")
    }

    /// URL redis
    pub fn redis_url(&self) -> Url {
        let url = format!("redis://{}:{}", self.redis_host, self.redis_port);
        Url::parse(&url).expect("Неправильно заданный адрес redis сервера")
    }
}

impl IEnvVars for Config {}

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use env_vars_lib::{load_env_vars, Errors};

/// Загружаем настройки из переменных окружения из файла .env
pub fn load_config() -> Result<Config, Errors> {
    load_env_vars()
}

/// Настройки системы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_ws_port: u16,

    // Для статической проверки SQL-запросов, не переименовывать
    pub database_url: Url,

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

    pub grafana_port: u16,

    pub loki_port: u16,
    pub loki_url: Url,

    pub portainer_port: u16,

    pub redis_channel: String,
    pub redis_port: u16,
    pub redis_port_ui: u16,
    pub redis_url: Url,

    pub webapp_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_ws_port: 8081,

            loki_port: 3100,
            loki_url: Url::from_str("http://localhost:3100").unwrap(),

            database_url: Url::from_str(
                "postgres://DB_USER:DB_PASSWORD@DB_HOST:5432/db_data_test",
            )
            .unwrap(),

            db_host: "localhost".to_string(),
            db_port: 5432,
            db_user: "postgres".to_string(),
            db_password: "postgres".to_string(),

            deconz_hub_api_key: "api_key".to_string(),
            deconz_hub_host: "localhost".to_string(),
            deconz_hub_port_api: 8010,
            deconz_hub_port_vnc: 8011,
            deconz_hub_port_ws: 8012,
            deconz_hub_vnc_password: "vnc_password".to_string(),

            grafana_port: 3000,

            portainer_port: 8001,

            redis_channel: Default::default(),
            redis_port: 6379,
            redis_port_ui: 8013,
            redis_url: Url::from_str("redis://localhost:6379").unwrap(),

            webapp_port: 8090,
        }
    }
}

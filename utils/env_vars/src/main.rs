use clap::Parser;
use tracing::{info, warn};

use env_vars::{create_env_file, Config};
use env_vars::{load, Er};
use env_vars_lib::{Cli, Commands};

fn main() {
    tracing_subscriber::fmt().init();
    let cli = Cli::parse();

    let value = match cli.command {
        Some(value) => value,
        None => todo!(),
    };
    match value {
        Commands::Create => command_create(),
        Commands::Check => command_check(),
    }
}

fn command_create() {
    create_env_file::<Config>(".env.example").unwrap()
}

fn command_check() -> Result {
    info!("Пробуем загрузить файл .env");
    let config = match load() {
        Ok(val) => val,
        Err(err) => {
            warn!("{:?}", err);
            return;
        }
    };
    info!("Загружены настройки: {:#?}", config);
}

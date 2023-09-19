use clap::Parser;
use tracing::info;

use env_vars_lib::{create_env_file, load_env_vars, Cli, Commands, Errors};

use env_vars::Config;

const ENV_EXAMPLE_FILE: &str = ".env.example";

fn main() {
    tracing_subscriber::fmt().init();
    let cli = Cli::parse();

    let value = match cli.command {
        Some(value) => value,
        None => todo!(),
    };
    let command = match value {
        Commands::Create => command_create(),
        Commands::Check => command_check(),
    };
    command.unwrap();
}

fn command_create() -> Result<(), Errors> {
    info!("Создаем файл {}", ENV_EXAMPLE_FILE);
    create_env_file::<Config>(ENV_EXAMPLE_FILE)?;
    info!("Файл {} создан", ENV_EXAMPLE_FILE);
    Ok(())
}

fn command_check() -> Result<(), Errors> {
    info!("Пробуем загрузить файл .env");
    let config = load_env_vars::<Config>()?;
    info!("Загружены настройки: {:#?}", config);
    Ok(())
}

//! CLI интерфейс

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Создать файл .env.example
    Create,
    /// Проверка файла .env на правильность конфигурации
    Check,
}

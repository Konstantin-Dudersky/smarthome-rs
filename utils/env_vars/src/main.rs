use rsiot_env_vars::env_vars_cli;

use env_vars::Config;

fn main() {
    tracing_subscriber::fmt().init();

    env_vars_cli::<Config>()
}

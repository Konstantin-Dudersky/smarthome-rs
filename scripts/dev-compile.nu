
export def main [] {
    cargo +nightly build --release -p webapp
    cargo build --target=aarch64-unknown-linux-gnu --release -p db-saver
    cargo build --target=aarch64-unknown-linux-gnu --release -p deconz-ws
    cargo build --target=aarch64-unknown-linux-gnu --release -p http-server
    cargo build --target=aarch64-unknown-linux-gnu --release -p websocket-server
}

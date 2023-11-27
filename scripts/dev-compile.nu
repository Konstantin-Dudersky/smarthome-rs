
export def main [] {
    cd services/webapp | npx tailwindcss -o ./style/output.css --minify | trunk build --release | cd ../..
    cargo build --target=aarch64-unknown-linux-gnu --release -p db-saver
    cargo build --target=aarch64-unknown-linux-gnu --release -p deconz-ws
    cargo build --target=aarch64-unknown-linux-gnu --release -p http-server
    cargo build --target=aarch64-unknown-linux-gnu --release -p websocket-server
}

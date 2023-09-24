export def main [] {
    docker compose up -d
    ~/.cargo/bin/pueue add ./target/deconz-ws
}

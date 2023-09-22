export def main [] {
    docker compose up -d;
    ./target/deconz-ws;
}

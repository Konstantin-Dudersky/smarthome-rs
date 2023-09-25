use lib/ui.nu

export def main [] {
    ui "Start docker"
    docker compose up -d

    sleep 5sec

    ui "Start services"
    ~/.cargo/bin/pueue add ./target/db-saver
    ~/.cargo/bin/pueue add ./target/deconz-ws
    ~/.cargo/bin/pueue parallel 2
}

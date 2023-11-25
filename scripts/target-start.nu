use lib/ui.nu

export def main [] {
    ui "Start docker"
    docker compose --profile target up -d

    sleep 2sec

    ui "Start services"
    sudo pueue add ./target/db-saver
    sudo pueue add ./target/deconz-ws
    sudo pueue add ./target/http-server
    sudo pueue add ./target/websocket-server
    sudo pueue parallel 4
}

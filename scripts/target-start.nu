use lib/ui.nu

export def main [] {
    ui "Start docker"
    docker compose --profile target up -d

    sleep 2sec

    ui "Start services"
    sudo pueue add ./db-saver
    sudo pueue add ./deconz-ws
    sudo pueue add ./http-server
    sudo pueue add ./websocket-server
    sudo pueue parallel 4
}

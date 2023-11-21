use lib/ui.nu

export def main [] {
    ui "Start docker"
    docker compose --profile target up -d

    sleep 5sec

    ui "Start services"
    sudo pueue add ./target/db-saver
    sudo pueue add ./target/deconz-ws
    sudo pueue parallel 2
}

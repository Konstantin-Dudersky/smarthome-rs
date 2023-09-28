use lib/ui.nu

export def main [] {
    ui "Start docker"
    docker compose up -d

    sleep 5sec

    ui "Start services"
    sudo pueue add ./target/db-saver
    sudo pueue add ./target/deconz-ws
    sudo pueue parallel 2

    # ~/.cargo/bin/pueue add ./target/db-saver
    # ~/.cargo/bin/pueue add ./target/deconz-ws
    # ~/.cargo/bin/pueue parallel 2
}

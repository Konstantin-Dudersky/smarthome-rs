use lib/ui.nu

export def main [] {
    ui "Stop services"
    sudo pueue reset -f

    ui "Stop docker"
    docker compose --profile target down
}

use lib/ui.nu

export def main [] {
    ui "Stop docker"
    docker compose --profile target down

    ui "Stop services"
    sudo pueue reset -f
}

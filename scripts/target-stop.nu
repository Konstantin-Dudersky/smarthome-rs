use lib/ui.nu

export def main [] {
    ui "Stop docker"
    docker compose down

    ui "Stop services"
    ~/.cargo/bin/pueue reset -f
}

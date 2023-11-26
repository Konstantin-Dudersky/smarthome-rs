use lib/ui.nu

# -vhra = --verbose --human-readable --recursive --archive 

# Синхронизация кода
export def main [] {
    ui "Синхронизация файлов"

    # бинарники
    rsync -vhra ./target/aarch64-unknown-linux-gnu/release/deconz-ws pi@target:~/smarthome/
    rsync -vhra ./target/aarch64-unknown-linux-gnu/release/db-saver pi@target:~/smarthome/
    rsync -vhra ./target/aarch64-unknown-linux-gnu/release/http-server pi@target:~/smarthome/
    rsync -vhra ./target/aarch64-unknown-linux-gnu/release/websocket-server pi@target:~/smarthome/

    # папки с конфигурациями
    rsync -vhra --relative ./services/db/ pi@target:~/smarthome/
    rsync -vhra --relative ./services/grafana/ pi@target:~/smarthome/
    rsync -vhra --relative ./services/webapp/dist/ pi@target:~/smarthome/
    rsync -vhra --relative ./scripts pi@target:~/smarthome/

    rsync -vhra --relative ./docker-compose.yml pi@target:~/smarthome/
}
# -vhra => --verbose --human-readable --recursive --archive 

use lib/ui.nu

# Синхронизация кода
export def main [] {
    ui "Синхронизация файлов"

    # бинарники
    rsync -vhra ./services/deconz-ws/target/aarch64-unknown-linux-gnu/release/deconz-ws pi@target:~/smarthome/
    rsync -vhra ./services/db-saver/target/aarch64-unknown-linux-gnu/release/db-saver pi@target:~/smarthome/
    rsync -vhra ./services/http-server/target/aarch64-unknown-linux-gnu/release/http-server pi@target:~/smarthome/
    rsync -vhra ./services/websocket-server/target/aarch64-unknown-linux-gnu/release/websocket-server pi@target:~/smarthome/

    # папки с конфигурациями
    rsync -vhra --relative ./services/db/ pi@target:~/smarthome/
    rsync -vhra --relative ./services/redis/ pi@target:~/smarthome/
    rsync -vhra --relative ./services/grafana/ pi@target:~/smarthome/
    rsync -vhra --relative ./services/webapp/dist/ pi@target:~/smarthome/
    rsync -vhra --relative ./scripts pi@target:~/smarthome/

    rsync -vhra --relative ./docker-compose.yml pi@target:~/smarthome/
}
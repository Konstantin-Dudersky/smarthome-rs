set dotenv-load
set shell := ["nu", "-c"]


default:
  just --list

# Синхронизация кода
sync:
    rsync -vhra . pi@192.168.101.10:~/code --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after

build-target:
    cargo build --release --target=aarch64-unknown-linux-gnu

deploy: sync

# останов
target-stop:
    docker compose down

# запуск
target-start:
    docker compose up -d

target-start-ssh:
    ssh pi@192.168.101.10 'cd ~/code; /home/pi/.cargo/bin/just'

    # ssh pi@192.168.101.10 "cd ~/code; just target-start"
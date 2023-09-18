set dotenv-load
set shell := ["nu", "-c"]


default:
  just --list

# Синхронизация кода
sync:
    rsync -vhra . pi@192.168.101.10:/mnt/external/code --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after

build-target:
    cargo build --release --target=aarch64-unknown-linux-gnu

deploy: sync

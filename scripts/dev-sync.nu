use lib/ui.nu

# Синхронизация кода
export def main [] {
    ui "Синхронизация файлов"

    # синхронизировать все файлы, как в git
    rsync -vhra . pi@target:~/code --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after

    rsync -vhra ./target/aarch64-unknown-linux-gnu/release/deconz-ws pi@target:~/code/target/

    rsync -vhra ./target/aarch64-unknown-linux-gnu/release/db-saver pi@target:~/code/target/
}

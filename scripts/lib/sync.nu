# Синхронизация кода
export def main [] {
    rsync -vhra . pi@target:~/code --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after
}

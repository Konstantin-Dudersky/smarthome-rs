# Запуск скрипта по ssh
export def main [script: string] {
    ssh pi@target $"cd ~/code; ~/.cargo/bin/nu scripts/($script).nu"
}

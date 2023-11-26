# Запуск скрипта по ssh
export def main [script: string] {
    ssh -t pi@target $"cd ~/smarthome; ~/.cargo/bin/nu scripts/($script).nu"
}

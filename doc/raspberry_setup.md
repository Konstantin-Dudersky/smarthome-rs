# Настройка Raspberry

Скачиваем и устанавливаем Ubuntu Server 23.04 64 bit

```bash
# обновление
sudo apt update && sudo apt upgrade
# rust -------------------------------------------------------------------------
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# nushell ----------------------------------------------------------------------
sudo apt -y install pkg-config libssl-dev build-essential
cargo install nu jazi

# добавить в путь в PATH
nano ~/.config/nushell/env.nu
# $env.PATH = ... append '/home/user/.cargo/bin'

# Дальше устанавливаем из скрипта
nu scripts/target-install.nu

# TODO создаем .env файл
```

## Systemd - дописать

```bash
sudo mv smarthome.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable smarthome
sudo systemctl start smarthome

# Проверить логи
journalctl -xeu smarthome.service
```

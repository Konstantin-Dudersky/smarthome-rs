# Настройка Raspberry

Скачиваем и устанавливаем Ubuntu Server 23.04 64 bit

```bash
# обновление
sudo apt update && sudo apt upgrade
# rust -------------------------------------------------------------------------
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# nushell ----------------------------------------------------------------------
sudo apt -y install pkg-config libssl-dev build-essential
cargo install nu

# установить стартовый шелл
sudo nano /etc/passwd
# найти строку, где задается шелл, заменить
# /bin/bash -> /home/pi/.cargo/bin/nu

# Дальше устанавливаем из скрипта
nu scripts/target-install.nu

# TODO создаем .env файл
```

# Настройка Raspberry

Скачиваем и устанавливаем Ubuntu Server 23.04 64 bit

```bash
# обновление
sudo apt update && sudo apt upgrade
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Установка nushell
sudo apt install pkg-config libssl-dev
sudo apt install build-essential
cargo install nu

# монтирование внешнего диска
# создаем папку
sudo mkdir /mnt/external
# находим UUID
sudo blkid
# добавляем в файл /etc/fstab
# UUID=__UUID__  /mnt/external  ext4  defaults  0  2
# проверка
sudo mount -a
```

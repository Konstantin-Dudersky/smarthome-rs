# Настройка Raspberry

Скачиваем и устанавливаем Ubuntu Server 23.04 64 bit

```bash
# обновление
sudo apt update && sudo apt upgrade
# rust -------------------------------------------------------------------------
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# nushell ----------------------------------------------------------------------
sudo apt install pkg-config libssl-dev
sudo apt install build-essential
cargo install nu just

# монтирование внешнего диска --------------------------------------------------
# создаем папку
sudo mkdir /mnt/external
# находим UUID
sudo blkid
# добавляем в файл /etc/fstab
# UUID=__UUID__  /mnt/external  ext4  defaults  0  2
# проверка
sudo mount -a

# docker -----------------------------------------------------------------------
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo docker run hello-world
# non-sudo
sudo groupadd docker
sudo usermod -aG docker $USER
newgrp docker
docker run hello-world
# autostart
sudo systemctl enable docker.service
sudo systemctl enable containerd.service
# change directory
sudo systemctl stop docker
sudo systemctl stop docker.socket
sudo systemctl stop containerd
sudo mkdir -p /mnt/external/docker
# отредактировать файл /etc/docker/daemon.json
# { "data-root": "/mnt/external/docker" }
sudo systemctl start docker
```

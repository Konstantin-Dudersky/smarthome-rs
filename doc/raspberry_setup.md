# Настройка Raspberry

Скачиваем и устанавливаем Ubuntu Server 23.04 64 bit

```bash
# обновление
sudo apt update && sudo apt upgrade
# rust -------------------------------------------------------------------------
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# nushell ----------------------------------------------------------------------
sudo apt -y install pkg-config libssl-dev
sudo apt -y install build-essential
cargo install nu just

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

# TODO копируем код, создаем .env файл
```

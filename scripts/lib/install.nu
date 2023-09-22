# Установка docker
export def docker [] {
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo docker run hello-world
    # non-sudo
    sudo groupadd docker
    sudo usermod -aG docker $env.USER
    newgrp docker
    run-external "docker" "run" "hello-world"
    # autostart
    sudo systemctl enable docker.service
    sudo systemctl enable containerd.service
}

# Настройка Raspberry

Ubuntu 23.04

```bash
sudo apt update && sudo apt upgrade
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Установка nushell
sudo apt install pkg-config libssl-dev
sudo apt install build-essential
cargo install nu
```

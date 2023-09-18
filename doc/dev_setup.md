# Настройка среды разработки

## Настрока кросс-компиляции

```nu
sudo apt install gcc-aarch64-linux-gnu;
rustup target add aarch64-unknown-linux-gnu
```

Создать файл ~/.cargo/config:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

Теперь можно собирать командой:

```bash
cargo build --target=aarch64-unknown-linux-gnu
```

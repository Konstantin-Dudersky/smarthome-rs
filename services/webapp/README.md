## Подготовка

Установить инструменты

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

Установить пакеты

```bash
npm install
```

В файле `.cargo/config.toml`:

```toml
[toolchain]
channel = "nightly"
targets = ["wasm32-unknown-unknown"]
```

Запуск для отладки:

```
trunk serve --open
```

Сборка в релиз:

```bash
npx tailwindcss -o ./style/output.css --minify | trunk build --release
```

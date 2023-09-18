Загрузка переменных среды из ОС и из файла .env

Список переменных задается в структуре Config в файле `src/config.rs`

## Команды

### Cоздать / обновить файл .env.example

```bash
cargo run --bin env_vars create
```

Создает файл `.env.example` со значениями по-умолчанию

### Проверить файл .env

```bash
cargo run --bin env_vars check
```

1. Пытается загрузить файл `.env`
2. Читает настройки в структуру `src/config.rs`

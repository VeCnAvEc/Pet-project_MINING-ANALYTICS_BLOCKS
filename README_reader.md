# RabbitMQ Stream Reader

Простой reader для чтения сообщений из RabbitMQ Stream.

## Запуск

```bash
# Способ 1: Через скрипт
./run_reader.sh

# Способ 2: Напрямую
export RUST_LOG=info
cargo run --bin main_reader
```

## Что делает

1. Подключается к RabbitMQ Stream на localhost:5552
2. Читает сообщения из потока "mining-analytics"
3. Выводит сообщения в консоль в формате JSON
4. Работает до нажатия Ctrl+C

## Настройка

Если нужно изменить настройки подключения, отредактируйте файл `src/main_reader.rs`:

```rust
let environment = Arc::new(
    Environment::builder()
        .host("localhost")  // Измените на ваш хост
        .port(5552)         // Измените на ваш порт
        .username("guest")  // Измените на вашего пользователя
        .password("guest")  // Измените на ваш пароль
        .build()
        .await?
);
```

## Остановка

Нажмите `Ctrl+C` для остановки reader. 
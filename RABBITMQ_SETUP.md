# Настройка RabbitMQ для проекта

## Установка RabbitMQ

### Ubuntu/Debian:
```bash
# Установка Erlang
sudo apt-get install erlang

# Установка RabbitMQ
sudo apt-get install rabbitmq-server

# Запуск сервиса
sudo systemctl start rabbitmq-server
sudo systemctl enable rabbitmq-server
```

### Docker:
```bash
docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management
```

## Настройка пользователя и прав доступа

```bash
# Создание пользователя
sudo rabbitmqctl add_user mining_user mining_password

# Назначение роли администратора
sudo rabbitmqctl set_user_tags mining_user administrator

# Назначение прав доступа
sudo rabbitmqctl set_permissions -p / mining_user ".*" ".*" ".*"
```

## Веб-интерфейс управления

RabbitMQ Management доступен по адресу: http://localhost:15672
- Логин: mining_user
- Пароль: mining_password

## Настройка в проекте

В файле `src/main.rs` измените URL подключения:

```rust
let amqp_url = "amqp://mining_user:mining_password@localhost:5672";
```

## Структура очередей

- `block_data_queue` - очередь для данных о блоках и coinbase транзакциях

## Мониторинг

Для мониторинга очередей используйте веб-интерфейс или команды:

```bash
# Просмотр очередей
sudo rabbitmqctl list_queues

# Просмотр соединений
sudo rabbitmqctl list_connections

# Просмотр каналов
sudo rabbitmqctl list_channels
``` 
use std::sync::Arc;
use anyhow::Result;
use log::info;
use rabbitmq_stream_client::Environment;
use rabbitmq_stream_client::types::OffsetSpecification;
use futures_util::stream::TryStreamExt;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логирования
    env_logger::init();

    // Конфигурация подключения к RabbitMQ
    let environment = Arc::new(
        Environment::builder()
            .host("localhost")
            .port(5552)
            .username("guest")
            .password("guest")
            .build()
            .await?
    );

    info!("Подключаемся к RabbitMQ Stream...");

    // Создаем consumer для чтения сообщений
    let mut consumer = environment
        .consumer()
        .name("test-reader")
        .offset(OffsetSpecification::First)
        .build("mining-analytics")
        .await?;

    let current_time: DateTime<Utc> = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S UTC");

    info!("Начинаем чтение сообщений из потока 'mining-analytics'...");
    info!("Нажмите Ctrl+C для остановки");

    // Читаем сообщения
    while let Ok(Some(delivery)) = consumer.try_next().await {
        let message = delivery.message();
        
        // Получаем данные из сообщения
        if let Some(data) = message.data() {
            // Пытаемся распарсить как JSON
            if let Ok(json_str) = std::str::from_utf8(data) {
                println!("[{}] === Сообщение ===", formatted_time);
                println!("{}", json_str);
                println!("================\n");
            } else {
                println!("[{}] === Бинарные данные ===", formatted_time);
                println!("{:?}", data);
                println!("======================\n");
            }
        } else {
            println!("[{}] === Сообщение без данных ===", formatted_time);
            println!("======================\n");
        }
    }

    Ok(())
} 
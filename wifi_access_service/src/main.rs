pub mod app;
pub mod event_queue;
pub mod container;

use container::Container;
use app::adapters::rabbitmq::RabbitMqAdapter;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new("wifi_access_serv=debug"))
        .with(fmt::layer())
        .init();

    // Подключаемся к RabbitMQ
    let rabbitmq_url = std::env::var("RABBITMQ_URL").unwrap_or_else(|_| "amqp://user:pass@rabbitmq:5672".into());
    let rabbitmq_adapter = RabbitMqAdapter::connect(&rabbitmq_url).await.unwrap();

    let container = Arc::new(Container::new(rabbitmq_adapter));

    // Запускаем consumer в фоне
    let container_clone = container.clone();
    tokio::spawn(async move {
        container_clone.rabbitmq_adapter.start_listening(container_clone.event_queue.clone()).await;
    });

    let container_clone2 = container.clone();
    // Тестовый loop, обрабатываем очередь каждые 5 секунд
    tokio::spawn({
        let container = container_clone2.clone();
        async move {
            loop {
                let ips: Vec<String> = {
                    let mut queue = container.event_queue.lock().unwrap();
                    queue.drain(..).collect()
                };

                for ip in ips {
                    app::command::add_user_to_wifi::send_to_mikrotik(ip).await;
                }

                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    });


    tracing::info!("wifi_access_serv запущен");
    // Сервис без HTTP, чисто очередь + обработка
    loop { sleep(Duration::from_secs(60)).await; }
}

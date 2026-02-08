use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Channel};
use std::sync::{Arc, Mutex};
use futures::StreamExt;

#[derive(Clone)]
pub struct RabbitMqAdapter {
    channel: Channel,
}

impl RabbitMqAdapter {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let conn = Connection::connect(url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        Ok(Self { channel })
    }

    pub async fn start_listening(&self, event_queue: Arc<Mutex<Vec<String>>>) {
        let queue_name = "wifi_user_queue";
        self.channel.queue_declare(
            queue_name,
            QueueDeclareOptions{
                durable: true,
                ..Default::default()
            },
             FieldTable::default()
        ).await
        .expect("Не удалось объявить очередь RabbitMQ");

        let mut consumer = self.channel.basic_consume(
            queue_name,
            "wifi_access_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await.unwrap();

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let msg = String::from_utf8(delivery.data.clone()).unwrap_or_default();
                tracing::info!("Получено сообщение из RabbitMQ: {}", msg);
                {
                    let mut q = event_queue.lock().unwrap();
                    q.push(msg);
                }
                self.channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await.unwrap();
            }
        }
    }
}

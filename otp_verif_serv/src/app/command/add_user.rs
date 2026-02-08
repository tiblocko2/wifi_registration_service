use crate::app::adapters::rabbitmq_producer::RabbitMqProducer;
use std::{net::IpAddr, sync::Arc};

pub struct WifiUser {
    ipaddr: IpAddr,
    wifi_producer: Arc<RabbitMqProducer>,
}

impl WifiUser {
    pub fn new(ipaddr: IpAddr, wifi_producer: Arc<RabbitMqProducer>) -> Self {
        Self { ipaddr, wifi_producer }
    }

    pub async fn execute(&self) {
        tracing::info!("Добавление WiFi пользователя с IP: {}", self.ipaddr);
        // 1. публикуем событие в очередь
        if let Err(err) = self.wifi_producer.publish_wifi_user(self.ipaddr).await {
            tracing::error!("Не удалось отправить событие в RabbitMQ: {:?}", err);
        }
    }
}
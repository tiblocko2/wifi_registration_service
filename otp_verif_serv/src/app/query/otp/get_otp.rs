use std::{sync::Arc, time::Duration};
use rand::{rng, Rng};

use crate::domain::otp::OtpEntry;
use crate::storage::repository::OtpRepository;
use crate::app::adapters::rabbitmq_producer::RabbitMqProducer;

pub struct GetOtpQuery {
    repo: Arc<dyn OtpRepository>,
    producer: Arc<RabbitMqProducer>,
}

impl GetOtpQuery {
    pub fn new(
        repo: Arc<dyn OtpRepository>,
        producer: Arc<RabbitMqProducer>,
    ) -> Self {
        Self { repo, producer }
    }

    pub async fn execute(&self, phone: String) {
        let code = rng().random_range(100000..999999).to_string();

        let entry = OtpEntry::new(
            phone.clone(),
            code,
            Duration::from_secs(300),
        );

        // 1. сохраняем OTP в БД
        self.repo.save(entry).await;

        // 2. публикуем событие в очередь
        if let Err(err) = self.producer.publish(phone).await {
            tracing::error!("Не удалось отправить событие в RabbitMQ: {:?}", err);
        }
    }
}
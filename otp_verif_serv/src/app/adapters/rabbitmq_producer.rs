use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties,
    Channel,
    Connection,
    ConnectionProperties,
};
use std::{net::IpAddr, sync::Arc};

pub struct RabbitMqProducer {
    channel: Arc<Channel>,
    queue: String,
}

impl RabbitMqProducer {
    pub async fn new(amqp_url: &str, queue: &str) -> anyhow::Result<Self> {
        let conn = Connection::connect(
            amqp_url,
            ConnectionProperties::default(),
        )
        .await?;

        let channel = conn.create_channel().await?;

        channel
            .queue_declare(
                queue,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        Ok(Self {
            channel: Arc::new(channel),
            queue: queue.to_string(),
        })
    }

    pub async fn publish(&self, phone: String) -> anyhow::Result<()> {
        self.channel
            .basic_publish(
                "",                 // default exchange
                &self.queue,        // routing key = имя очереди
                BasicPublishOptions::default(),
                phone.as_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?; // confirm

        Ok(())
    }

    pub async fn publish_wifi_user(&self, ipaddr: IpAddr) -> anyhow::Result<()> {
        self.channel
            .basic_publish(
                "",                 // default exchange
                &self.queue,        // routing key = имя очереди
                BasicPublishOptions::default(),
                ipaddr.to_string().as_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?; // confirm

        Ok(())
    }
}
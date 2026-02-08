use std::sync::{Arc, Mutex};
use crate::app::adapters::rabbitmq::RabbitMqAdapter;

pub struct Container {
    pub event_queue: Arc<Mutex<Vec<String>>>,
    pub rabbitmq_adapter: RabbitMqAdapter,
}

impl Container {
    pub fn new(rabbitmq_adapter: RabbitMqAdapter) -> Self {
        Self {
            event_queue: Arc::new(Mutex::new(Vec::new())),
            rabbitmq_adapter,
        }
    }
}

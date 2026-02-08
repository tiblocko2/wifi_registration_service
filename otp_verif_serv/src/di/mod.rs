use std::sync::Arc;

use crate::app::query::otp::{
    get_otp::GetOtpQuery,
    verify_otp::VerifyOtpQuery,
};

use crate::storage::repository::OtpRepository;
use crate::app::adapters::rabbitmq_producer::RabbitMqProducer;

pub struct Container {
    pub get_otp_query: GetOtpQuery,
    pub verify_otp_query: VerifyOtpQuery,
    pub wifi_producer: Arc<RabbitMqProducer>,
}

impl Container {
    pub fn new(
        repository: Arc<dyn OtpRepository>,
        producer: Arc<RabbitMqProducer>,
        wifi_producer: Arc<RabbitMqProducer>,
    ) -> Self {
        Self {
            get_otp_query: GetOtpQuery::new(
                repository.clone(),
                producer,
            ),
            verify_otp_query: VerifyOtpQuery::new(repository),
            wifi_producer,
        }
    }
}
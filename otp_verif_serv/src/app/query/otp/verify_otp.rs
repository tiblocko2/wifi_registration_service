use std::sync::Arc;

use crate::domain::otp::OtpStatus;
use crate::storage::repository::OtpRepository;

pub struct VerifyOtpQuery {
    repo: Arc<dyn OtpRepository>,
}

impl VerifyOtpQuery {
    pub fn new(repo: Arc<dyn OtpRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, phone: String, code: String) -> OtpStatus {
        match self.repo.find(&phone).await{
            None => OtpStatus::InvalidCode,
            Some(entry) if entry.is_expired() => {
                self.repo.remove(&phone).await;
                OtpStatus::Expired
            }
            Some(entry) if entry.code == code => {
                self.repo.remove(&phone).await;
                OtpStatus::Verified
            }
            Some(_) => OtpStatus::InvalidCode,
        }
    }
}
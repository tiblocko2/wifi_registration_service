use crate::domain::otp::OtpEntry;


#[async_trait::async_trait]
pub trait OtpRepository: Send + Sync {
    async fn save(&self, entry: OtpEntry);
    async fn find(&self, phone: &str) -> Option<OtpEntry>;
    async fn remove(&self, phone: &str);
}

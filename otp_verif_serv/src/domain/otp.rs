use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct OtpEntry {
    pub phone: String,
    pub code: String,
    pub expires_at: Instant,
}

#[derive(Debug, PartialEq)]
pub enum OtpStatus {
    Verified,
    InvalidCode,
    Expired,
}

impl OtpEntry {
    pub fn new(phone: String, code: String, ttl: Duration) -> Self {
        Self {
            phone,
            code,
            expires_at: Instant::now() + ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::OtpEntry;

    /// Проверяем, что OTP сразу после создания НЕ считается просроченным
    #[tokio::test]
    async fn otp_is_not_expired_initially() {
        // Given
        let entry = OtpEntry::new(
            "79990000000".into(),
            "123456".into(),
            Duration::from_secs(60),
        );

        // When
        let expired = entry.is_expired();

        // Then
        assert!(!expired, "Новый OTP не должен быть просрочен");
    }

    /// Проверяем, что OTP считается просроченным после истечения TTL
    #[tokio::test]
    async fn otp_is_expired_after_ttl() {
        // Given
        let mut entry = OtpEntry::new(
            "79990000000".into(),
            "123456".into(),
            Duration::from_secs(60),
        );

        entry.expires_at = Instant::now() - Duration::from_secs(1);

        // When
        let expired = entry.is_expired();

        // Then
        assert!(expired, "OTP должен считаться просроченным");
    }
}


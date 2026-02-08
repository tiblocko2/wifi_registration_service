use dashmap::DashMap;
use std::sync::Arc;

use crate::domain::otp::OtpEntry;
use crate::storage::repository::OtpRepository;

#[derive(Clone)]
pub struct InMemoryOtpRepository {
    store: Arc<DashMap<String, OtpEntry>>,
}

impl InMemoryOtpRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }
}

impl OtpRepository for InMemoryOtpRepository {
    fn save(&self, entry: OtpEntry) {
        self.store.insert(entry.phone.clone(), entry);
    }

    fn find(&self, phone: &str) -> Option<OtpEntry> {
        self.store.get(phone).map(|v| v.clone())
    }

    fn remove(&self, phone: &str) {
        self.store.remove(phone);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::domain::otp::OtpEntry;

    /// Тест: сохранение OTP и успешное получение по номеру телефона
    #[tokio::test]
    async fn save_and_find_otp() {
        // Given
        let repository = InMemoryOtpRepository::new();
        let otp = OtpEntry::new(
            "79990000000".to_string(),
            "123456".to_string(),
            Duration::from_secs(60),
        );

        // When
        repository.save(otp.clone());

        // Then
        let result = repository.find("79990000000");
        assert!(result.is_some(), "OTP должен быть найден в репозитории");
        let stored = result.unwrap();
        assert_eq!(stored.phone, "79990000000");
        assert_eq!(stored.code, "123456");
    }

    /// Тест: попытка получить OTP по несуществующему номеру
    #[tokio::test]
    async fn find_returns_none_for_unknown_phone() {
        // Given
        let repository = InMemoryOtpRepository::new();

        // When
        let result = repository.find("79990000000");

        // Then
        assert!(result.is_none(), "Для неизвестного номера должен вернуться None");
    }

    /// Тест: удаление OTP из репозитория
    #[tokio::test]
    async fn remove_otp() {
        // Given
        let repository = InMemoryOtpRepository::new();
        let otp = OtpEntry::new(
            "79990000000".to_string(),
            "654321".to_string(),
            Duration::from_secs(60),
        );
        repository.save(otp);

        // When
        repository.remove("79990000000");
        let result = repository.find("79990000000");

        // Then
        assert!(result.is_none(), "OTP должен быть удалён из репозитория");
    }

    /// Тест: повторное сохранение OTP с тем же номером перезаписывает старый
    #[tokio::test]
    async fn save_overwrites_existing_otp() {
        // Given
        let repository = InMemoryOtpRepository::new();

        let first = OtpEntry::new(
            "79990000000".to_string(),
            "111111".to_string(),
            Duration::from_secs(60),
        );

        let second = OtpEntry::new(
            "79990000000".to_string(),
            "222222".to_string(),
            Duration::from_secs(60),
        );

        // When
        repository.save(first);
        repository.save(second);
        let result = repository.find("79990000000");

        // Then
        assert!(result.is_some(), "OTP должен существовать");
        assert_eq!(
            result.unwrap().code,
            "222222",
            "OTP должен быть перезаписан новым значением"
        );
    }
}

use crate::domain::otp::OtpEntry;
use crate::storage::repository::OtpRepository;
use sqlx::PgPool;
use std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use sqlx::Row;
use std::time::Instant;

#[derive(Clone)]
pub struct PostgresOtpRepository {
    pool: Arc<PgPool>,
}

impl PostgresOtpRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl OtpRepository for PostgresOtpRepository {
    async fn save(&self, entry: OtpEntry) {
        let phone = entry.phone.clone();
        let code = entry.code.clone();
        let expires_at = (Utc::now() + Duration::seconds(300)).naive_utc();

        let _ = sqlx::query(
            r#"
            INSERT INTO otp (phone, code, expires_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (phone) DO UPDATE
            SET code = EXCLUDED.code, expires_at = EXCLUDED.expires_at
            "#,
        )
        .bind(phone)
        .bind(code)
        .bind(expires_at)
        .execute(&*self.pool)
        .await;
    }

    async fn find(&self, phone: &str) -> Option<OtpEntry> {
        let row = sqlx::query(
            r#"
            SELECT phone, code, expires_at
            FROM otp
            WHERE phone = $1
            "#,
        )
        .bind(phone)
        .fetch_optional(&*self.pool)
        .await
        .ok()??;

        let expires_at: DateTime<Utc> = row.get("expires_at");
        let now = Utc::now();

        let ttl = if expires_at > now {
            (expires_at - now).to_std().ok()?
        } else {
            std::time::Duration::from_secs(0)
        };

        Some(OtpEntry {
            phone: row.get("phone"),
            code: row.get("code"),
            expires_at: Instant::now() + ttl,
        })
    }

    async fn remove(&self, phone: &str) {
        let _ = sqlx::query("DELETE FROM otp WHERE phone = $1")
            .bind(phone)
            .execute(&*self.pool)
            .await;
    }
}


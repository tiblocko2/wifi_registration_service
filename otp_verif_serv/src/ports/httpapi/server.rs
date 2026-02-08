use axum::{
    Json, Router, ServiceExt, extract::{ConnectInfo, State}, routing::post
};
use tokio::net::TcpListener;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

use crate::di::Container;
use crate::domain::otp::OtpStatus;
use crate::app::command::add_user;

#[derive(Deserialize)]
pub struct RequestOtpDto {
    pub phone: String,
}

#[derive(Deserialize)]
pub struct VerifyOtpDto {
    pub phone: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}

pub struct Server {
    port: u16,
    container: Arc<Container>,
}

impl Server {
    pub fn new(port: u16, container: Arc<Container>) -> Self {
        Self { port, container }
    }

    pub async fn run(self) {
        // Настройка трассировки
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "otp_verif_serv=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .try_init()
            .ok();

        let app = get_router(self.container.clone());

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        tracing::info!("Listening on 0.0.0.0:{}", self.port);
        


        // Запуск сервера
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    }
}

async fn request_otp_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<RequestOtpDto>,
) -> Json<StatusResponse> {
    // Логируем событие
    tracing::info!("Запрос /otp/request для номера {}", payload.phone);

    container.get_otp_query.execute(payload.phone).await;

    tracing::info!("OTP для сохранён в БД");

    Json(StatusResponse {
        status: "code_generated".into(),
    })
}

async fn verify_otp_handler(
    State(container): State<Arc<Container>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyOtpDto>,
) -> Json<StatusResponse> {
    tracing::info!("Запрос /otp/verify для номера {}", payload.phone);

    let result = container
        .verify_otp_query
        .execute(payload.phone.to_owned(), payload.code)
        .await;

    tracing::info!("Результат проверки для {}: {:?}", payload.phone, result);
    
    if result == OtpStatus::Verified {
        let ip = addr.ip();
        let wifi_user = add_user::WifiUser::new(ip, container.wifi_producer.clone());
        wifi_user.execute().await;
    }
    

    let status = match result {
        OtpStatus::Verified => "verified",
        OtpStatus::InvalidCode => "invalid_code",
        OtpStatus::Expired => "expired",
    };

    Json(StatusResponse {
        status: status.into(),
    })
}

pub fn get_router(container: Arc<Container>) -> Router {
    Router::new()
        .route("/otp/request", post(request_otp_handler))
        .route("/otp/verify", post(verify_otp_handler))
        .with_state(container)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}
pub async fn send_to_mikrotik(ip: String) {
    // Тут можно подключиться к API MikroTik (через TCP 8728) и добавить в firewall
    tracing::info!("(MOCK) Добавляем IP {} в MikroTik firewall", ip);

    // Реальный пример через tcp / API RouterOS можно будет добавить
    // Пример:
    // let mut conn = RouterOsApi::connect(host, user, pass).await?;
    // conn.command("/ip/firewall/address-list/add")
    //     .param("list", "wifi_allowed")
    //     .param("address", ip)
    //     .send().await?;
}

use tokio::net::TcpListener;

pub async fn listener_config() -> Result<TcpListener, Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: std::net::SocketAddr =
        std::net::SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>()?));

    let listener: TcpListener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    Ok(listener)
}

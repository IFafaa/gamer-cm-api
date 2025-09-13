use tower_http::cors::{Any, CorsLayer};

pub fn cors_config() -> Result<CorsLayer, Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Ok(cors)
}

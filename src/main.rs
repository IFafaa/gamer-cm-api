use shared::state::AppState;

pub mod domain;
pub mod infra;
pub mod presentation;
pub mod shared;
pub mod application;

#[tokio::main]
async fn main() {
    infra::configs::envs_config::config_env().expect("Failed to load environment variables");

    let cors = infra::configs::cors_config::cors_config().expect("Failed to configure CORS");

    let db = infra::configs::db_config::config_database()
        .await
        .expect("Failed to connect to the database");

    let listener = infra::configs::listener_config::listener_config()
        .await
        .expect("Failed to bind to address");

    let app_state = AppState { db };

    let app = presentation::route::create_routes(app_state).layer(cors);

    let addr = listener.local_addr().expect("Failed to get local address");
    println!("🚀 Server started successfully!");
    println!("🌐 Listening on http://{}", addr);
    println!("📚 API Documentation available at http://{}/api-docs/", addr);
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

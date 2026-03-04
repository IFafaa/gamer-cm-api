use axum::Router;

use crate::infra::configs::swagger_config::create_swagger_ui;
use crate::presentation::middleware::auth_middleware::auth_middleware;
use crate::shared::state::AppState;

use super::routes::auth_routes;
use super::routes::community_routes;
use super::routes::party_routes;
use super::routes::player_routes;
use super::routes::stats_routes;
use super::routes::team_routes;

pub fn create_routes(app_state: AppState) -> Router {
    let protected_routes = Router::new()
        .nest("/communities", community_routes::community_routes())
        .nest("/players", player_routes::player_routes())
        .nest("/teams", team_routes::team_routes())
        .nest("/parties", party_routes::party_routes())
        .nest("/stats", stats_routes::stats_routes())
        .with_state(app_state.clone())
        .layer(axum::middleware::from_fn(auth_middleware));

    let api_routes = Router::new()
        .nest(
            "/auth",
            auth_routes::auth_routes().with_state(app_state.clone()),
        )
        .merge(protected_routes);

    Router::new().merge(create_swagger_ui()).merge(api_routes)
}

use axum::Router;

use crate::shared::state::AppState;
use crate::infra::configs::swagger_config::create_swagger_ui;

use super::routes::community_routes;
use super::routes::party_routes;
use super::routes::player_routes;
use super::routes::team_routes;

pub fn create_routes(app_state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/communities", community_routes::community_routes())
        .nest("/players", player_routes::player_routes())
        .nest("/teams", team_routes::team_routes())
        .nest("/parties", party_routes::party_routes())
        .with_state(app_state);

    Router::new()
        .merge(create_swagger_ui())
        .merge(api_routes)
}

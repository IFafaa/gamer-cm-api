use axum::{Json, http::StatusCode};
use serde::Deserialize;

use crate::shared::api_error::ApiErrorResponse;

#[derive(Debug, Deserialize)]
pub struct DeletePlayersOfTeamDto {
    pub team_id: i32,
    pub name: Option<String>,
    pub player_ids: Vec<i32>,
}

impl DeletePlayersOfTeamDto {
    pub fn validate(&self) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
        let mut errors: Vec<String> = Vec::new();

        if self.team_id <= 0 {
            errors.push("Team ID must be greater than 0".into());
        }

        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                errors.push("Team name cannot be empty".into());
            }
            if name.len() > 50 {
                errors.push("Team name must be 50 characters or less".into());
            }
        }

        if self.player_ids.iter().any(|id| *id <= 0) {
            errors.push("All player IDs must be greater than 0".into());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse::new(errors.join(", "))),
            ))
        }
    }
}

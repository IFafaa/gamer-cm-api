use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{
    application::interfaces::result_get_stats_interface::{
        CommunityStats, GameStats, PlayerRanking, TeamRanking,
    },
    domain::community::CommunityRepository,
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};
use std::sync::Arc;

pub struct GetCommunityStatsUseCase<CR: CommunityRepository> {
    community_repository: Arc<CR>,
    pool: PgPool,
}

impl<CR: CommunityRepository> GetCommunityStatsUseCase<CR> {
    pub fn new(community_repository: Arc<CR>, pool: PgPool) -> Self {
        Self {
            community_repository,
            pool,
        }
    }

    pub async fn execute(
        &self,
        community_id: i32,
        user_id: i32,
    ) -> Result<ApiResponse<CommunityStats>, (StatusCode, ApiErrorResponse)> {
        let community = self
            .community_repository
            .get_by_id_and_user(community_id, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Community not found".to_string()),
            ))?;

        let total_players = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM players WHERE community_id = $1 AND enabled = true",
            community_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch stats".to_string()),
            )
        })?
        .unwrap_or(0);

        let total_teams = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM teams WHERE community_id = $1 AND enabled = true",
            community_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch stats".to_string()),
            )
        })?
        .unwrap_or(0);

        let total_parties = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM parties WHERE community_id = $1 AND enabled = true",
            community_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch stats".to_string()),
            )
        })?
        .unwrap_or(0);

        let finished_parties = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM parties WHERE community_id = $1 AND enabled = true AND finished_at IS NOT NULL",
            community_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch stats".to_string()),
            )
        })?
        .unwrap_or(0);

        let active_parties = total_parties - finished_parties;

        // Team rankings: wins and total parties played
        let team_rows = sqlx::query!(
            r#"
            SELECT 
                t.id as team_id,
                t.name as team_name,
                COUNT(DISTINCT pt.party_id) as total_parties,
                COUNT(DISTINCT CASE WHEN p.team_winner_id = t.id THEN p.id END) as wins
            FROM teams t
            LEFT JOIN party_teams pt ON pt.team_id = t.id
            LEFT JOIN parties p ON p.id = pt.party_id AND p.enabled = true
            WHERE t.community_id = $1 AND t.enabled = true
            GROUP BY t.id, t.name
            ORDER BY wins DESC, total_parties DESC
            "#,
            community_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch team rankings".to_string()),
            )
        })?;

        let team_rankings: Vec<TeamRanking> = team_rows
            .into_iter()
            .map(|r| {
                let wins = r.wins.unwrap_or(0);
                let total = r.total_parties.unwrap_or(0);
                let win_rate = if total > 0 {
                    (wins as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                TeamRanking {
                    team_id: r.team_id,
                    team_name: r.team_name,
                    wins,
                    total_parties: total,
                    win_rate: (win_rate * 100.0).round() / 100.0,
                }
            })
            .collect();

        // Player rankings: wins based on being on the winning team
        let player_rows = sqlx::query!(
            r#"
            SELECT 
                pl.id as player_id,
                pl.nickname as player_nickname,
                COUNT(DISTINCT pt.party_id) as total_parties,
                COUNT(DISTINCT CASE WHEN p.team_winner_id = tp.team_id THEN p.id END) as wins
            FROM players pl
            INNER JOIN team_players tp ON tp.player_id = pl.id AND tp.enabled = true
            INNER JOIN party_teams pt ON pt.team_id = tp.team_id
            INNER JOIN parties p ON p.id = pt.party_id AND p.enabled = true
            WHERE pl.community_id = $1 AND pl.enabled = true
            GROUP BY pl.id, pl.nickname
            ORDER BY wins DESC, total_parties DESC
            "#,
            community_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch player rankings".to_string()),
            )
        })?;

        let player_rankings: Vec<PlayerRanking> = player_rows
            .into_iter()
            .map(|r| {
                let wins = r.wins.unwrap_or(0);
                let total = r.total_parties.unwrap_or(0);
                let win_rate = if total > 0 {
                    (wins as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                PlayerRanking {
                    player_id: r.player_id,
                    player_nickname: r.player_nickname,
                    wins,
                    total_parties: total,
                    win_rate: (win_rate * 100.0).round() / 100.0,
                }
            })
            .collect();

        // Most played games
        let game_rows = sqlx::query!(
            r#"
            SELECT game_name, COUNT(*) as times_played
            FROM parties
            WHERE community_id = $1 AND enabled = true AND game_name IS NOT NULL
            GROUP BY game_name
            ORDER BY times_played DESC
            LIMIT 10
            "#,
            community_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to fetch game stats".to_string()),
            )
        })?;

        let most_played_games: Vec<GameStats> = game_rows
            .into_iter()
            .map(|r| GameStats {
                game_name: r.game_name.unwrap_or_default(),
                times_played: r.times_played.unwrap_or(0),
            })
            .collect();

        let stats = CommunityStats {
            community_id: community.id,
            community_name: community.name,
            total_players,
            total_teams,
            total_parties,
            active_parties,
            finished_parties,
            team_rankings,
            player_rankings,
            most_played_games,
        };

        Ok(ApiResponse::success(stats))
    }
}

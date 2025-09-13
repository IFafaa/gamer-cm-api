use crate::domain::{
    player::Player,
    team::{Team, TeamRepository},
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgTeamRepository {
    pub pool: PgPool,
}

impl PgTeamRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn fetch_team_players(
        &self,
        community_id: i32,
        team_id: i32,
    ) -> anyhow::Result<Vec<Player>> {
        let rows = sqlx::query!(
            "SELECT p.id, p.nickname, p.community_id, p.created_at, p.updated_at, p.enabled
             FROM players p
             INNER JOIN team_players tp ON p.id = tp.player_id
             WHERE p.community_id = $1 AND tp.team_id = $2 AND tp.enabled = true AND p.enabled = true",
            community_id,
            team_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|p| Player {
                id: p.id,
                nickname: p.nickname,
                community_id: p.community_id,
                created_at: p.created_at,
                updated_at: p.updated_at,
                enabled: p.enabled,
            })
            .collect())
    }
}

#[async_trait]
impl TeamRepository for PgTeamRepository {
    async fn get_by_ids(&self, ids: Vec<i32>) -> anyhow::Result<Vec<Team>> {
        let rows = sqlx::query!(
            "SELECT id, name, community_id, created_at, updated_at, enabled FROM teams WHERE id = ANY($1)",
            &ids
        )
        .fetch_all(&self.pool)
        .await?;

        let mut teams = Vec::new();
        for row in rows {
            let players = self.fetch_team_players(row.community_id, row.id).await?;

            teams.push(Team {
                id: row.id,
                name: row.name,
                community_id: row.community_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: row.enabled,
                players,
            });
        }

        Ok(teams)
    }

    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Team>> {
        let team = sqlx::query! {
            "SELECT id, name, community_id, created_at, updated_at, enabled FROM teams WHERE id = $1",
            id
        }
        .fetch_optional(&self.pool)
        .await?;

        if let Some(team_row) = team {
            let players = self
                .fetch_team_players(team_row.community_id, team_row.id)
                .await?;

            return Ok(Some(Team {
                id: team_row.id,
                name: team_row.name,
                community_id: team_row.community_id,
                created_at: team_row.created_at,
                updated_at: team_row.updated_at,
                enabled: team_row.enabled,
                players,
            }));
        }

        Ok(None)
    }

    async fn insert(&self, team: &Team) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO teams (name, community_id) VALUES ($1, $2)",
            team.name,
            team.community_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM teams WHERE name = $1 AND community_id = $2)",
            name,
            community_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.exists.unwrap_or(false))
    }

    async fn save(&self, team: &Team) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE teams SET name = $1, enabled = $2, updated_at = NOW() WHERE id = $3",
            team.name,
            team.enabled,
            team.id
        )
        .execute(&self.pool)
        .await?;

        // Atualizar todos os players do time
        for player in &team.players {
            sqlx::query!(
                "INSERT INTO team_players (team_id, player_id, created_at, updated_at, enabled)
                 VALUES ($1, $2, NOW(), NOW(), $3)
                 ON CONFLICT (team_id, player_id)
                 DO UPDATE SET enabled = $3, updated_at = NOW()",
                team.id,
                player.id,
                player.enabled
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

}

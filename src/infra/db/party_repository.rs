use anyhow::Ok;
use async_trait::async_trait;
use sqlx::{PgPool, types::JsonValue};
use time::PrimitiveDateTime;

use crate::domain::{
    party::{IGetPartiesByParams, Party, PartyRepository},
    player::Player,
    team::Team,
};

pub struct PgPartyRepository {
    pub pool: PgPool,
}

impl PgPartyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn fetch_team_players(
        &self,
        community_id: i32,
        team_id: i32,
    ) -> anyhow::Result<Vec<Player>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, nickname, community_id, created_at, updated_at 
            FROM players 
            WHERE community_id = $1 
              AND enabled = true 
              AND id IN (
                  SELECT player_id FROM team_players WHERE team_id = $2
              )
            "#,
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
                enabled: true,
            })
            .collect())
    }

    fn build_dynamic_query(
        base_query: &str,
        community_id: Option<i32>,
        game_name: Option<String>,
        created_at: Option<PrimitiveDateTime>,
        updated_at: Option<PrimitiveDateTime>,
        teams_ids: Option<&Vec<i32>>,
        team_winner_ids: Option<&Vec<i32>>,
    ) -> (String, Vec<JsonValue>) {
        let mut conditions = vec![];
        let mut params: Vec<JsonValue> = Vec::new();
        let mut idx = 1;

        macro_rules! push_cond {
            ($opt:expr, $cond:expr, $value:expr) => {
                if let Some(val) = $opt {
                    conditions.push(format!($cond, idx));
                    params.push(serde_json::to_value(val).unwrap().into());
                    idx += 1;
                }
            };
        }

        push_cond!(community_id, "community_id = ${}", community_id.unwrap());
        push_cond!(game_name, "game_name = ${}", game_name.unwrap());
        push_cond!(created_at, "created_at = ${}", created_at.unwrap());
        push_cond!(updated_at, "updated_at = ${}", updated_at.unwrap());

        if let Some(ids) = teams_ids {
            if !ids.is_empty() {
                conditions.push(format!(
                    "id IN (SELECT party_id FROM party_teams WHERE team_id = ANY(${}))",
                    idx
                ));
                params.push(serde_json::to_value(ids).unwrap().into());
                idx += 1;
            }
        }

        if let Some(ids) = team_winner_ids {
            if !ids.is_empty() {
                conditions.push(format!("team_winner_id = ANY(${})", idx));
                params.push(serde_json::to_value(ids).unwrap().into());
            }
        }

        let where_clause = if conditions.is_empty() {
            "".to_string()
        } else {
            format!("AND {}", conditions.join(" AND "))
        };

        (format!("{base_query} {where_clause}"), params)
    }
}

#[async_trait]
impl PartyRepository for PgPartyRepository {
    async fn insert(&self, party: &Party) -> anyhow::Result<()> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO parties (community_id, game_name, team_winner_id) 
            VALUES ($1, $2, $3) 
            RETURNING id
            "#,
            party.community_id,
            party.game_name,
            party.team_winner_id,
        )
        .fetch_one(&self.pool)
        .await?;

        for team in &party.teams {
            sqlx::query!(
                "INSERT INTO party_teams (party_id, team_id) VALUES ($1, $2)",
                rec.id,
                team.id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn get_by_params(&self, params: IGetPartiesByParams) -> anyhow::Result<Vec<Party>> {
        let community_id = params.community_id;
        let game_name = params.game_name;
        let created_at = params.created_at;
        let updated_at = params.updated_at;
        let teams_ids = params.teams_ids;
        let team_winner_ids = params.team_winner_ids;

        let base_query = r#"
            SELECT id, community_id, game_name, team_winner_id, finished_at, created_at, updated_at, enabled 
            FROM parties
            WHERE enabled = true
        "#;

        let (query, params) = Self::build_dynamic_query(
            base_query,
            community_id,
            game_name,
            created_at,
            updated_at,
            teams_ids.as_ref(),
            team_winner_ids.as_ref(),
        );

        let mut q = sqlx::query_as::<
            _,
            (
                i32,
                i32,
                String,
                Option<i32>,
                Option<PrimitiveDateTime>,
                PrimitiveDateTime,
                PrimitiveDateTime,
                bool,
            ),
        >(&query);
        for param in params {
            q = q.bind(param);
        }

        let party_rows = q.fetch_all(&self.pool).await?;

        let mut parties = Vec::new();

        for (
            id,
            community_id,
            game_name,
            team_winner_id,
            finished_at,
            created_at,
            updated_at,
            enabled,
        ) in party_rows
        {
            let team_rows = sqlx::query!(
                r#"
                SELECT t.id, t.community_id, t.enabled, t.name, t.created_at, t.updated_at 
                FROM teams t 
                INNER JOIN party_teams pt ON pt.team_id = t.id 
                WHERE pt.party_id = $1 AND t.enabled = true
                "#,
                id
            )
            .fetch_all(&self.pool)
            .await?;

            let mut teams = Vec::new();
            for row in team_rows {
                let players = self.fetch_team_players(row.community_id, row.id).await?;
                teams.push(Team {
                    id: row.id,
                    community_id: row.community_id,
                    enabled: row.enabled,
                    name: row.name,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    players,
                });
            }

            parties.push(Party {
                id,
                community_id,
                game_name,
                team_winner_id,
                finished_at,
                created_at,
                updated_at,
                teams,
                enabled,
            });
        }

        Ok(parties)
    }

    async fn get_by_community_id(&self, community_id: i32) -> anyhow::Result<Vec<Party>> {
        let party_rows = sqlx::query!(
            r#"
            SELECT id, community_id, game_name, team_winner_id, finished_at, created_at, updated_at, enabled
            FROM parties
            WHERE community_id = $1 AND enabled = true
            ORDER BY created_at DESC
            "#,
            community_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut parties = Vec::new();

        for row in party_rows {
            let team_rows = sqlx::query!(
                r#"
                SELECT t.id, t.community_id, t.enabled, t.name, t.created_at, t.updated_at 
                FROM teams t 
                INNER JOIN party_teams pt ON pt.team_id = t.id 
                WHERE pt.party_id = $1 AND t.enabled = true
                "#,
                row.id
            )
            .fetch_all(&self.pool)
            .await?;

            let mut teams = Vec::new();
            for t in team_rows {
                let players = self.fetch_team_players(t.community_id, t.id).await?;
                teams.push(Team {
                    id: t.id,
                    community_id: t.community_id,
                    enabled: t.enabled,
                    name: t.name,
                    created_at: t.created_at,
                    updated_at: t.updated_at,
                    players,
                });
            }

            parties.push(Party {
                id: row.id,
                community_id: row.community_id,
                game_name: row.game_name.unwrap_or_default(),
                team_winner_id: row.team_winner_id,
                finished_at: row.finished_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
                teams,
                enabled: row.enabled,
            });
        }

        Ok(parties)
    }

    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Party>> {
        let row = sqlx::query!(
            r#"
            SELECT id, community_id, game_name, team_winner_id, finished_at, created_at, updated_at, enabled
            FROM parties
            WHERE id = $1 AND enabled = true
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let team_rows = sqlx::query!(
                r#"
                SELECT t.id, t.community_id, t.enabled, t.name, t.created_at, t.updated_at 
                FROM teams t 
                INNER JOIN party_teams pt ON pt.team_id = t.id 
                WHERE pt.party_id = $1 AND t.enabled = true
                "#,
                row.id
            )
            .fetch_all(&self.pool)
            .await?;

            let mut teams = Vec::new();
            for t in team_rows {
                let players = self.fetch_team_players(t.community_id, t.id).await?;
                teams.push(Team {
                    id: t.id,
                    community_id: t.community_id,
                    enabled: t.enabled,
                    name: t.name,
                    created_at: t.created_at,
                    updated_at: t.updated_at,
                    players,
                });
            }

            Ok(Some(Party {
                id: row.id,
                community_id: row.community_id,
                game_name: row.game_name.unwrap_or_default(),
                team_winner_id: row.team_winner_id,
                finished_at: row.finished_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
                teams,
                enabled: row.enabled,
            }))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, party: &Party) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE parties 
            SET game_name = $1, team_winner_id = $2, finished_at = $3, enabled = $4, updated_at = $5
            WHERE id = $6
            "#,
            party.game_name,
            party.team_winner_id,
            party.finished_at,
            party.enabled,
            party.updated_at,
            party.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::player::Player;

#[derive(Serialize, Clone)]

pub struct Team {
    pub id: i32,
    pub name: String,
    pub players: Vec<Player>,
    pub community_id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Team {
    pub fn new(name: String, community_id: i32) -> Self {
        Team {
            id: 0,
            name,
            community_id,
            players: Vec::new(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
        self.updated_at = DateTime::now();
    }
}

#[async_trait::async_trait]

pub trait TeamRepository: Send + Sync {
    async fn insert(&self, team: &Team) -> anyhow::Result<()>;
    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Team>>;
    async fn get_by_ids(&self, ids: Vec<i32>) -> anyhow::Result<Vec<Team>>;
    async fn save(&self, team: &Team) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::player::Player;

    fn make_player(id: i32) -> Player {
        let mut p = Player::new(format!("player_{}", id), 1);
        p.id = id;
        p
    }

    #[test]
    fn test_team_new_sets_fields() {
        let team = Team::new("Alpha Squad".to_string(), 7);
        assert_eq!(team.name, "Alpha Squad");
        assert_eq!(team.community_id, 7);
        assert_eq!(team.id, 0);
        assert!(team.players.is_empty());
        assert!(team.enabled);
    }

    #[test]
    fn test_team_add_player_increments_list() {
        let mut team = Team::new("Alpha Squad".to_string(), 1);
        assert!(team.players.is_empty());
        team.add_player(make_player(1));
        assert_eq!(team.players.len(), 1);
        team.add_player(make_player(2));
        assert_eq!(team.players.len(), 2);
    }

    #[test]
    fn test_team_add_player_stores_correct_nickname() {
        let mut team = Team::new("Squad".to_string(), 1);
        team.add_player(make_player(10));
        assert_eq!(team.players[0].nickname, "player_10");
    }
}

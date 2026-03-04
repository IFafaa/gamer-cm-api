use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

#[derive(Serialize, Clone)]
pub struct Player {
    pub id: i32,
    pub nickname: String,
    pub community_id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Player {
    pub fn new(nickname: String, community_id: i32) -> Self {
        Player {
            id: 0,
            nickname,
            community_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = DateTime::now();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[async_trait::async_trait]
pub trait PlayerRepository: Send + Sync {
    async fn insert(&self, player: &Player) -> anyhow::Result<()>;
    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Player>>;
    async fn get_by_ids(&self, ids: Vec<i32>) -> anyhow::Result<Vec<Player>>;
    async fn save(&self, player: &Player) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new_sets_fields() {
        let player = Player::new("xXSniper99Xx".to_string(), 3);
        assert_eq!(player.nickname, "xXSniper99Xx");
        assert_eq!(player.community_id, 3);
        assert_eq!(player.id, 0);
        assert!(player.enabled);
    }

    #[test]
    fn test_player_is_enabled_by_default() {
        let player = Player::new("nick".to_string(), 1);
        assert!(player.is_enabled());
    }

    #[test]
    fn test_player_disable() {
        let mut player = Player::new("nick".to_string(), 1);
        player.disable();
        assert!(!player.is_enabled());
        assert!(!player.enabled);
    }

    #[test]
    fn test_player_disable_idempotent() {
        let mut player = Player::new("nick".to_string(), 1);
        player.disable();
        player.disable();
        assert!(!player.is_enabled());
    }
}

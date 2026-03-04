use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::{player::Player, team::Team};

#[derive(Serialize)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Community {
    pub fn new(name: String, user_id: i32) -> Self {
        Community {
            id: 0,
            name,
            user_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            players: Vec::new(),
            teams: Vec::new(),
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
pub trait CommunityRepository: Send + Sync {
    async fn insert(&self, community: &Community) -> anyhow::Result<()>;
    async fn exists(&self, name: String, user_id: i32) -> anyhow::Result<bool>;
    async fn get_all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<Community>>;
    async fn get_by_id_and_user(&self, id: i32, user_id: i32) -> anyhow::Result<Option<Community>>;
    async fn belongs_to_user(&self, community_id: i32, user_id: i32) -> anyhow::Result<bool>;
    async fn get_ids_by_user(&self, user_id: i32) -> anyhow::Result<Vec<i32>>;
    async fn save(&self, community: &Community) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_community_new_sets_fields() {
        let community = Community::new("Gaming Hub".to_string(), 42);
        assert_eq!(community.name, "Gaming Hub");
        assert_eq!(community.user_id, 42);
        assert_eq!(community.id, 0);
        assert!(community.enabled);
        assert!(community.players.is_empty());
        assert!(community.teams.is_empty());
    }

    #[test]
    fn test_community_is_enabled_by_default() {
        let community = Community::new("Test".to_string(), 1);
        assert!(community.is_enabled());
    }

    #[test]
    fn test_community_disable_sets_enabled_false() {
        let mut community = Community::new("Test".to_string(), 1);
        community.disable();
        assert!(!community.is_enabled());
        assert!(!community.enabled);
    }

    #[test]
    fn test_community_disable_idempotent() {
        let mut community = Community::new("Test".to_string(), 1);
        community.disable();
        community.disable();
        assert!(!community.is_enabled());
    }
}

use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::team::Team;

pub struct Party {
    pub id: i32,
    pub community_id: i32,
    pub game_name: String,
    pub teams: Vec<Team>,
    pub team_winner_id: Option<i32>,
    pub finished_at: Option<PrimitiveDateTime>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Party {
    pub fn new(game_name: String, teams: Vec<Team>, community_id: i32) -> Self {
        Party {
            id: 0,
            community_id,
            teams,
            team_winner_id: None,
            game_name,
            finished_at: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    pub fn end(&mut self, winner_team_id: Option<i32>) {
        self.team_winner_id = winner_team_id;
        self.finished_at = Some(DateTime::now());
        self.updated_at = DateTime::now();
    }

    pub fn is_finished(&self) -> bool {
        self.finished_at.is_some()
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
pub trait PartyRepository: Send + Sync {
    async fn insert(&self, party: &Party) -> anyhow::Result<()>;
    async fn get_by_params(&self, params: IGetPartiesByParams) -> anyhow::Result<Vec<Party>>;
    async fn get_by_community_id(&self, community_id: i32) -> anyhow::Result<Vec<Party>>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Party>>;
    async fn save(&self, party: &Party) -> anyhow::Result<()>;
}

pub struct IGetPartiesByParams {
    pub community_id: Option<i32>,
    pub game_name: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
    pub teams_ids: Option<Vec<i32>>,
    pub team_winner_ids: Option<Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::team::Team;

    fn make_team(id: i32, name: &str) -> Team {
        let mut team = Team::new(name.to_string(), 1);
        team.id = id;
        team
    }

    #[test]
    fn test_party_new_sets_fields() {
        let teams = vec![make_team(1, "A"), make_team(2, "B")];
        let party = Party::new("CS:GO".to_string(), teams, 10);
        assert_eq!(party.game_name, "CS:GO");
        assert_eq!(party.community_id, 10);
        assert_eq!(party.teams.len(), 2);
        assert_eq!(party.id, 0);
        assert!(party.team_winner_id.is_none());
        assert!(party.finished_at.is_none());
        assert!(party.enabled);
    }

    #[test]
    fn test_party_is_not_finished_initially() {
        let party = Party::new("Valorant".to_string(), vec![], 1);
        assert!(!party.is_finished());
    }

    #[test]
    fn test_party_end_with_winner() {
        let mut party = Party::new("Valorant".to_string(), vec![], 1);
        party.end(Some(5));
        assert!(party.is_finished());
        assert_eq!(party.team_winner_id, Some(5));
        assert!(party.finished_at.is_some());
    }

    #[test]
    fn test_party_end_without_winner() {
        let mut party = Party::new("Valorant".to_string(), vec![], 1);
        party.end(None);
        assert!(party.is_finished());
        assert!(party.team_winner_id.is_none());
        assert!(party.finished_at.is_some());
    }

    #[test]
    fn test_party_disable() {
        let mut party = Party::new("Dota 2".to_string(), vec![], 1);
        assert!(party.is_enabled());
        party.disable();
        assert!(!party.is_enabled());
        assert!(!party.enabled);
    }
}

use axum::http::StatusCode;

use crate::{
    domain::{community::CommunityRepository, party::PartyRepository, team::TeamRepository},
    presentation::dtos::end_party_dto::EndPartyDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct EndPartyUseCase<PR: PartyRepository, TR: TeamRepository, CR: CommunityRepository> {
    party_repository: Arc<PR>,
    team_repository: Arc<TR>,
    community_repository: Arc<CR>,
}

impl<PR: PartyRepository, TR: TeamRepository, CR: CommunityRepository>
    EndPartyUseCase<PR, TR, CR>
{
    pub fn new(
        party_repository: Arc<PR>,
        team_repository: Arc<TR>,
        community_repository: Arc<CR>,
    ) -> Self {
        Self {
            party_repository,
            team_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: EndPartyDto,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let party = self
            .party_repository
            .get_by_id(dto.party_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch party".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Party not found".to_string()),
            ))?;

        let belongs = self
            .community_repository
            .belongs_to_user(party.community_id, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to verify ownership".to_string()),
                )
            })?;

        if !belongs {
            return Err((
                StatusCode::FORBIDDEN,
                ApiErrorResponse::new("Party does not belong to user".to_string()),
            ));
        }

        if party.is_finished() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Party is already finished".to_string()),
            ));
        }

        let team_winner = match dto.team_winner_id {
            Some(team_id) => Some(
                self.team_repository
                    .get_by_id(team_id)
                    .await
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ApiErrorResponse::new("Failed to fetch team".to_string()),
                        )
                    })?
                    .ok_or((
                        StatusCode::BAD_REQUEST,
                        ApiErrorResponse::new("Team not found".to_string()),
                    ))?,
            ),
            None => None,
        };

        if let Some(ref team) = team_winner {
            let is_team_not_in_party = !party.teams.iter().any(|t| t.id == team.id);
            if is_team_not_in_party {
                return Err((
                    StatusCode::BAD_REQUEST,
                    ApiErrorResponse::new("Team not part of the party".to_string()),
                ));
            }
        }

        let mut party = party;
        let winner_id = team_winner.map(|team| team.id);
        party.end(winner_id);

        self.party_repository.save(&party).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to save party".to_string()),
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        community::Community, party::{IGetPartiesByParams, Party}, team::Team,
    };
    use mockall::mock;

    mock! {
        pub PartyRepo {}
        #[async_trait::async_trait]
        impl PartyRepository for PartyRepo {
            async fn insert(&self, party: &Party) -> anyhow::Result<()>;
            async fn get_by_params(&self, params: IGetPartiesByParams) -> anyhow::Result<Vec<Party>>;
            async fn get_by_community_id(&self, community_id: i32) -> anyhow::Result<Vec<Party>>;
            async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Party>>;
            async fn save(&self, party: &Party) -> anyhow::Result<()>;
        }
    }

    mock! {
        pub TeamRepo {}
        #[async_trait::async_trait]
        impl TeamRepository for TeamRepo {
            async fn insert(&self, team: &Team) -> anyhow::Result<()>;
            async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool>;
            async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Team>>;
            async fn get_by_ids(&self, ids: Vec<i32>) -> anyhow::Result<Vec<Team>>;
            async fn save(&self, team: &Team) -> anyhow::Result<()>;
        }
    }

    mock! {
        pub CommunityRepo {}
        #[async_trait::async_trait]
        impl CommunityRepository for CommunityRepo {
            async fn insert(&self, community: &Community) -> anyhow::Result<()>;
            async fn exists(&self, name: String, user_id: i32) -> anyhow::Result<bool>;
            async fn get_all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<Community>>;
            async fn get_by_id_and_user(&self, id: i32, user_id: i32) -> anyhow::Result<Option<Community>>;
            async fn belongs_to_user(&self, community_id: i32, user_id: i32) -> anyhow::Result<bool>;
            async fn get_ids_by_user(&self, user_id: i32) -> anyhow::Result<Vec<i32>>;
            async fn save(&self, community: &Community) -> anyhow::Result<()>;
        }
    }

    fn make_party_with_teams(teams: Vec<Team>) -> Party {
        Party::new("CS:GO".to_string(), teams, 1)
    }

    fn make_team(id: i32) -> Team {
        let mut t = Team::new(format!("Team {}", id), 1);
        t.id = id;
        t
    }

    fn make_finished_party() -> Party {
        let mut p = Party::new("Valorant".to_string(), vec![], 1);
        p.end(None);
        p
    }

    #[tokio::test]
    async fn test_end_party_without_winner_success() {
        let party = make_party_with_teams(vec![]);
        let mut party_repo = MockPartyRepo::new();
        party_repo.expect_get_by_id().returning(move |_| Ok(Some(make_party_with_teams(vec![]))));
        party_repo.expect_save().returning(|_| Ok(()));

        let mut community_repo = MockCommunityRepo::new();
        community_repo.expect_belongs_to_user().returning(|_, _| Ok(true));

        let team_repo = MockTeamRepo::new();

        let use_case = EndPartyUseCase::new(Arc::new(party_repo), Arc::new(team_repo), Arc::new(community_repo));
        let dto = EndPartyDto { party_id: 1, team_winner_id: None };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_ok());
        drop(party);
    }

    #[tokio::test]
    async fn test_end_party_not_found_returns_not_found() {
        let mut party_repo = MockPartyRepo::new();
        party_repo.expect_get_by_id().returning(|_| Ok(None));

        let use_case = EndPartyUseCase::new(
            Arc::new(party_repo),
            Arc::new(MockTeamRepo::new()),
            Arc::new(MockCommunityRepo::new()),
        );
        let dto = EndPartyDto { party_id: 99, team_winner_id: None };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_end_party_not_owned_returns_forbidden() {
        let mut party_repo = MockPartyRepo::new();
        party_repo.expect_get_by_id().returning(|_| Ok(Some(make_party_with_teams(vec![]))));

        let mut community_repo = MockCommunityRepo::new();
        community_repo.expect_belongs_to_user().returning(|_, _| Ok(false));

        let use_case = EndPartyUseCase::new(
            Arc::new(party_repo),
            Arc::new(MockTeamRepo::new()),
            Arc::new(community_repo),
        );
        let dto = EndPartyDto { party_id: 1, team_winner_id: None };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_end_already_finished_party_returns_bad_request() {
        let mut party_repo = MockPartyRepo::new();
        party_repo.expect_get_by_id().returning(|_| Ok(Some(make_finished_party())));

        let mut community_repo = MockCommunityRepo::new();
        community_repo.expect_belongs_to_user().returning(|_, _| Ok(true));

        let use_case = EndPartyUseCase::new(
            Arc::new(party_repo),
            Arc::new(MockTeamRepo::new()),
            Arc::new(community_repo),
        );
        let dto = EndPartyDto { party_id: 1, team_winner_id: None };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_end_party_winner_not_in_party_returns_bad_request() {
        let team_in_party = make_team(1);
        let team_winner = make_team(99); // not in party

        let party = make_party_with_teams(vec![team_in_party]);
        let mut party_repo = MockPartyRepo::new();
        let party_clone = Party::new("CS:GO".to_string(), vec![make_team(1)], 1);
        party_repo.expect_get_by_id().returning(move |_| Ok(Some(Party::new("CS:GO".to_string(), vec![make_team(1)], 1))));

        let mut community_repo = MockCommunityRepo::new();
        community_repo.expect_belongs_to_user().returning(|_, _| Ok(true));

        let mut team_repo = MockTeamRepo::new();
        team_repo.expect_get_by_id().returning(move |_| Ok(Some(make_team(99))));

        let use_case = EndPartyUseCase::new(
            Arc::new(party_repo),
            Arc::new(team_repo),
            Arc::new(community_repo),
        );
        let dto = EndPartyDto { party_id: 1, team_winner_id: Some(99) };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST);
        drop(party);
        drop(team_winner);
        drop(party_clone);
    }
}

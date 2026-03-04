use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_party_interface::IResultGetParty,
    domain::{community::CommunityRepository, party::PartyRepository},
    shared::{
        api_error::ApiErrorResponse, api_response::ApiResponse, pagination::PaginationParams,
    },
};

pub struct GetPartiesUseCase<PR: PartyRepository, CR: CommunityRepository> {
    party_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PartyRepository, CR: CommunityRepository> GetPartiesUseCase<PR, CR> {
    pub fn new(party_repository: Arc<PR>, community_repository: Arc<CR>) -> Self {
        Self {
            party_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: i32,
        community_id: Option<i32>,
        pagination: PaginationParams,
    ) -> Result<ApiResponse<Vec<IResultGetParty>>, (StatusCode, ApiErrorResponse)> {
        // If community_id filter provided, verify ownership
        if let Some(cid) = community_id {
            let belongs = self
                .community_repository
                .belongs_to_user(cid, user_id)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to verify community ownership".to_string()),
                    )
                })?;

            if !belongs {
                return Err((
                    StatusCode::FORBIDDEN,
                    ApiErrorResponse::new("Community does not belong to user".to_string()),
                ));
            }
        }

        // Get user's community IDs to filter parties
        let community_ids = if let Some(cid) = community_id {
            vec![cid]
        } else {
            self.community_repository
                .get_ids_by_user(user_id)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to fetch communities".to_string()),
                    )
                })?
        };

        if community_ids.is_empty() {
            let meta = pagination.meta(0);
            return Ok(ApiResponse::with_pagination(vec![], meta));
        }

        let mut all_parties = Vec::new();
        for cid in community_ids {
            let parties = self
                .party_repository
                .get_by_community_id(cid)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to fetch parties".to_string()),
                    )
                })?;

            all_parties.extend(parties);
        }

        all_parties.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let total = all_parties.len();
        let meta = pagination.meta(total);
        let paginated_parties = pagination.apply(all_parties);

        let result = paginated_parties
            .into_iter()
            .map(IResultGetParty::new)
            .collect();

        Ok(ApiResponse::with_pagination(result, meta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        community::Community,
        party::Party,
        party::IGetPartiesByParams,
    };
    use mockall::mock;

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

    fn make_party(id: i32, community_id: i32, game: &str) -> Party {
        let mut p = Party::new(game.to_string(), vec![], community_id);
        p.id = id;
        p
    }

    #[tokio::test]
    async fn returns_paginated_parties_for_community() {
        let mut party_repo = MockPartyRepo::new();
        let mut community_repo = MockCommunityRepo::new();

        community_repo
            .expect_belongs_to_user()
            .returning(|_, _| Ok(true));
        party_repo
            .expect_get_by_community_id()
            .returning(|_| {
                Ok((1..=5).map(|i| make_party(i, 1, &format!("Game{}", i))).collect())
            });

        let use_case = GetPartiesUseCase::new(Arc::new(party_repo), Arc::new(community_repo));
        let pagination = PaginationParams { page: 1, limit: 3 };
        let result = use_case.execute(1, Some(1), pagination).await;
        assert!(result.is_ok(), "Expected Ok result");
        let json = serde_json::to_value(result.ok().unwrap()).unwrap();
        assert_eq!(json["data"].as_array().unwrap().len(), 3);
        assert_eq!(json["meta"]["total"], 5);
        assert_eq!(json["meta"]["total_pages"], 2);
        assert_eq!(json["meta"]["has_next_page"], true);
    }

    #[tokio::test]
    async fn returns_empty_when_user_has_no_communities() {
        let party_repo = MockPartyRepo::new();
        let mut community_repo = MockCommunityRepo::new();

        community_repo
            .expect_get_ids_by_user()
            .returning(|_| Ok(vec![]));

        let use_case = GetPartiesUseCase::new(Arc::new(party_repo), Arc::new(community_repo));
        let pagination = PaginationParams::default();
        let result = use_case.execute(1, None, pagination).await;
        assert!(result.is_ok(), "Expected Ok result");
        let json = serde_json::to_value(result.ok().unwrap()).unwrap();
        assert_eq!(json["data"].as_array().unwrap().len(), 0);
        assert_eq!(json["meta"]["total"], 0);
    }

    #[tokio::test]
    async fn returns_forbidden_when_community_does_not_belong_to_user() {
        let party_repo = MockPartyRepo::new();
        let mut community_repo = MockCommunityRepo::new();

        community_repo
            .expect_belongs_to_user()
            .returning(|_, _| Ok(false));

        let use_case = GetPartiesUseCase::new(Arc::new(party_repo), Arc::new(community_repo));
        let pagination = PaginationParams::default();
        let result = use_case.execute(1, Some(99), pagination).await;

        assert!(result.is_err());
        let (status, _) = result.err().unwrap();
        assert_eq!(status, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn returns_second_page_correctly() {
        let mut party_repo = MockPartyRepo::new();
        let mut community_repo = MockCommunityRepo::new();

        community_repo
            .expect_belongs_to_user()
            .returning(|_, _| Ok(true));
        party_repo.expect_get_by_community_id().returning(|_| {
            Ok((1..=6).map(|i| make_party(i, 1, &format!("Game{}", i))).collect())
        });

        let use_case = GetPartiesUseCase::new(Arc::new(party_repo), Arc::new(community_repo));
        let pagination = PaginationParams { page: 2, limit: 4 };
        let result = use_case.execute(1, Some(1), pagination).await;
        assert!(result.is_ok(), "Expected Ok result");
        let json = serde_json::to_value(result.ok().unwrap()).unwrap();
        assert_eq!(json["data"].as_array().unwrap().len(), 2);
        assert_eq!(json["meta"]["total"], 6);
        assert_eq!(json["meta"]["has_next_page"], false);
        assert_eq!(json["meta"]["has_previous_page"], true);
    }
}

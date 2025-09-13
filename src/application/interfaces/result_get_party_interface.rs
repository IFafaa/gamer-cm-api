use serde::Serialize;

use crate::domain::party::Party;

#[derive(Serialize)]
pub struct IResultGetParty {
    id: i32,
    community_id: i32,
    game_name: String,
    team_winner_id: Option<i32>,
    created_at: String,
    updated_at: String,
    finished_at: Option<String>,
    teams: Vec<IResultPartyTeam>,
}

#[derive(Serialize)]
struct IResultPartyTeam {
    id: i32,
    name: String,
    players: Vec<IResultPartyPlayer>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct IResultPartyPlayer {
    id: i32,
    nickname: String,
    created_at: String,
    updated_at: String,
}

impl IResultGetParty {
    pub fn new(party: Party) -> Self {
        IResultGetParty {
            id: party.id,
            game_name: party.game_name,
            team_winner_id: party.team_winner_id,
            community_id: party.community_id,
            created_at: party.created_at.to_string(),
            updated_at: party.updated_at.to_string(),
            finished_at: party.finished_at.map(|date| date.to_string()),
            teams: party
                .teams
                .into_iter()
                .map(|team| IResultPartyTeam {
                    id: team.id,
                    name: team.name,
                    players: team
                        .players
                        .into_iter()
                        .map(|player| IResultPartyPlayer {
                            id: player.id,
                            nickname: player.nickname,
                            created_at: player.created_at.to_string(),
                            updated_at: player.updated_at.to_string(),
                        })
                        .collect(),
                    created_at: team.created_at.to_string(),
                    updated_at: team.updated_at.to_string(),
                })
                .collect(),
        }
    }
}

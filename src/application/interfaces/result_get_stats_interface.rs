use serde::Serialize;

#[derive(Serialize)]
pub struct CommunityStats {
    pub community_id: i32,
    pub community_name: String,
    pub total_players: i64,
    pub total_teams: i64,
    pub total_parties: i64,
    pub active_parties: i64,
    pub finished_parties: i64,
    pub team_rankings: Vec<TeamRanking>,
    pub player_rankings: Vec<PlayerRanking>,
    pub most_played_games: Vec<GameStats>,
}

#[derive(Serialize)]
pub struct TeamRanking {
    pub team_id: i32,
    pub team_name: String,
    pub wins: i64,
    pub total_parties: i64,
    pub win_rate: f64,
}

#[derive(Serialize)]
pub struct PlayerRanking {
    pub player_id: i32,
    pub player_nickname: String,
    pub wins: i64,
    pub total_parties: i64,
    pub win_rate: f64,
}

#[derive(Serialize)]
pub struct GameStats {
    pub game_name: String,
    pub times_played: i64,
}

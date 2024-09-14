use pandascore::model::{
    league::League, matches::Match, player::Player, series::Series, team::Team,
    tournament::Tournament, VideoGame, Winner,
};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    League(League),
    Series(Series),
    Tournament(Box<Tournament>),
    Match(Box<Match>),
    Team(Team),
    Player(Player),
    VideoGame(VideoGame),
    Winner(Winner),
    WinnerTeam { id: u64, team: Team },
    WinnerPlayer { id: u64, player: Player },
}

use std::sync::Arc;

use pandascore::{endpoint::all, model::Winner, ClientTransport};
use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, VertexIterator,
};

use super::vertex::Vertex;
use crate::adapter::AdapterInner;

pub(super) fn resolve_league_edge<'a, V: AsVertex<Vertex> + 'a>(
    adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "series" => league::series(adapter, contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'League'")
        }
    }
}

mod league {
    use std::sync::Arc;

    use pandascore::{
        endpoint::{all, CollectionOptions},
        ClientTransport,
    };
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use super::super::vertex::Vertex;
    use crate::{adapter::AdapterInner, pagination::PaginationIterator};

    pub(super) fn series<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_league()
                .expect("conversion failed, vertex was not a League");
            let id = vertex.id;
            Box::new(
                PaginationIterator::new(
                    Arc::clone(&adapter),
                    CollectionOptions::default(),
                    move |adapter, next| {
                        adapter.execute(
                            all::leagues::ListLeagueSeries::builder()
                                .id(id)
                                .options(next)
                                .build(),
                        )
                    },
                )
                .map(Vertex::Series),
            )
        })
    }
}

pub(super) fn resolve_series_edge<'a, V: AsVertex<Vertex> + 'a>(
    adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "tournaments" => series::tournaments(adapter, contexts),
        "winner" => series::winner(adapter, contexts),
        "league" => series::league(adapter, contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Series'")
        }
    }
}

mod series {
    use std::sync::Arc;

    use pandascore::{
        endpoint::{all, CollectionOptions},
        ClientTransport,
    };
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::{
        adapter::{edges::resolve_winner, AdapterInner, Vertex},
        pagination::PaginationIterator,
    };

    pub(super) fn tournaments<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_series()
                .expect("conversion failed, vertex was not a Series");
            let id = vertex.id;
            Box::new(
                PaginationIterator::new(
                    Arc::clone(&adapter),
                    CollectionOptions::default(),
                    move |adapter, next| {
                        adapter.execute(
                            all::series::ListSeriesTournaments::builder()
                                .id(id)
                                .options(next)
                                .build(),
                        )
                    },
                )
                .map(|x| Vertex::Tournament(Box::new(x))),
            )
        })
    }

    pub(super) fn winner<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_series()
                .expect("conversion failed, vertex was not a Series");
            let adapter = Arc::clone(&adapter);
            resolve_winner(&vertex.winner, adapter)
        })
    }

    pub(super) fn league<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_series()
                .expect("conversion failed, vertex was not a Series");
            let adapter = Arc::clone(&adapter);
            Box::new(
                adapter
                    .execute(all::leagues::GetLeague::from(vertex.league.id))
                    .map(Vertex::League)
                    .into_iter(),
            )
        })
    }
}

pub(super) fn resolve_tournament_edge<'a, V: AsVertex<Vertex> + 'a>(
    adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "teams" => tournament::teams(adapter, contexts),
        "matches" => tournament::matches(adapter, contexts),
        "video_game" => tournament::video_game(contexts),
        "winner" => tournament::winner(adapter, contexts),
        "league" => tournament::league(adapter, contexts),
        "series" => tournament::series(adapter, contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Tournament'")
        }
    }
}

mod tournament {
    use std::sync::Arc;

    use pandascore::{
        endpoint::{all, CollectionOptions},
        ClientTransport,
    };
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::{
        adapter::{edges::resolve_winner, AdapterInner, Vertex},
        pagination::PaginationIterator,
    };

    pub(super) fn teams<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_tournament()
                .expect("conversion failed, vertex was not a Tournament");
            let id = vertex.id;
            Box::new(
                PaginationIterator::new(
                    Arc::clone(&adapter),
                    CollectionOptions::default(),
                    move |adapter, next| {
                        adapter.execute(
                            all::tournament::ListTournamentTeams::builder()
                                .id(id)
                                .options(next)
                                .build(),
                        )
                    },
                )
                .map(Vertex::Team),
            )
        })
    }

    pub(super) fn matches<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_tournament()
                .expect("conversion failed, vertex was not a Tournament");
            let id = vertex.id;
            Box::new(
                PaginationIterator::new(
                    Arc::clone(&adapter),
                    CollectionOptions::default(),
                    move |adapter, next| {
                        adapter.execute(
                            all::tournament::ListTournamentMatches::builder()
                                .id(id)
                                .options(next)
                                .build(),
                        )
                    },
                )
                .map(|x| Vertex::Match(Box::new(x))),
            )
        })
    }

    pub(super) fn video_game<'a, V>(
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_tournament()
                .expect("conversion failed, vertex was not a Tournament");
            Box::new(std::iter::once(Vertex::VideoGame(
                vertex.video_game.clone(),
            )))
        })
    }

    pub(super) fn winner<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_tournament()
                .expect("conversion failed, vertex was not a Tournament");
            let adapter = Arc::clone(&adapter);
            resolve_winner(&vertex.winner, adapter)
        })
    }

    pub(super) fn league<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_tournament()
                .expect("conversion failed, vertex was not a Tournament");
            let adapter = Arc::clone(&adapter);
            Box::new(
                adapter
                    .execute(all::leagues::GetLeague::from(vertex.league.id))
                    .map(Vertex::League)
                    .into_iter(),
            )
        })
    }

    pub(super) fn series<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_tournament()
                .expect("conversion failed, vertex was not a Tournament");
            let adapter = Arc::clone(&adapter);
            Box::new(
                adapter
                    .execute(all::series::GetSeries::from(vertex.serie.id))
                    .map(Vertex::Series)
                    .into_iter(),
            )
        })
    }
}

pub(super) fn resolve_team_edge<'a, V: AsVertex<Vertex> + 'a>(
    adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "players" => team::players(adapter, contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Team'")
        }
    }
}

mod team {
    use std::sync::Arc;

    use pandascore::{endpoint::all, ClientTransport};
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::adapter::{AdapterInner, Vertex};

    pub(super) fn players<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_team()
                .expect("conversion failed, vertex was not a Series");
            let adapter = Arc::clone(&adapter);
            Box::new(
                vertex
                    .players
                    .clone()
                    .into_iter()
                    .filter_map(move |x| adapter.execute(all::players::GetPlayer::from(x.id)))
                    .map(Vertex::Player),
            )
        })
    }
}

pub(super) fn resolve_player_edge<'a, V: AsVertex<Vertex> + 'a>(
    adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "current_team" => player::current_team(adapter, contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Player'")
        }
    }
}

mod player {
    use std::sync::Arc;

    use pandascore::{endpoint::all, ClientTransport};
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::adapter::{AdapterInner, Vertex};

    pub(super) fn current_team<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_player()
                .expect("conversion failed, vertex was not a Player");
            let adapter = Arc::clone(&adapter);
            let id = vertex.current_team.as_ref().map(|t| t.id);
            Box::new(
                id.and_then(|id| {
                    adapter
                        .execute(all::teams::GetTeam::from(id))
                        .map(Vertex::Team)
                })
                .into_iter(),
            )
        })
    }
}

pub(super) fn resolve_match_edge<'a, V: AsVertex<Vertex> + 'a>(
    adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "winner" => r#match::winner(adapter, contexts),
        "league" => r#match::league(adapter, contexts),
        "series" => r#match::series(adapter, contexts),
        "tournament" => r#match::tournament(adapter, contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Match'")
        }
    }
}

mod r#match {
    use std::sync::Arc;

    use pandascore::{endpoint::all, ClientTransport};
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::adapter::{edges::resolve_winner, AdapterInner, Vertex};

    pub(super) fn winner<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_match()
                .expect("conversion failed, vertex was not a Match");
            let adapter = Arc::clone(&adapter);
            resolve_winner(&vertex.winner, adapter)
        })
    }

    pub(super) fn league<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_match()
                .expect("conversion failed, vertex was not a Match");
            let adapter = Arc::clone(&adapter);
            Box::new(
                adapter
                    .execute(all::leagues::GetLeague::from(vertex.league.id))
                    .map(Vertex::League)
                    .into_iter(),
            )
        })
    }

    pub(super) fn series<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_match()
                .expect("conversion failed, vertex was not a Match");
            let adapter = Arc::clone(&adapter);
            Box::new(
                adapter
                    .execute(all::series::GetSeries::from(vertex.serie.id))
                    .map(Vertex::Series)
                    .into_iter(),
            )
        })
    }

    pub(super) fn tournament<'a, V>(
        adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_match()
                .expect("conversion failed, vertex was not a Match");
            let adapter = Arc::clone(&adapter);
            Box::new(
                adapter
                    .execute(all::tournament::GetTournament::from(vertex.tournament_id))
                    .map(|t| Vertex::Tournament(Box::new(t)))
                    .into_iter(),
            )
        })
    }
}

pub(super) fn resolve_winner_team_edge<'a, V: AsVertex<Vertex> + 'a>(
    _adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "team" => winner_team::team(contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Match'")
        }
    }
}

mod winner_team {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::adapter::Vertex;

    pub(super) fn team<'a, V>(
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_winner_team()
                .expect("conversion failed, vertex was not a WinnerTeam");
            Box::new(std::iter::once(Vertex::Team(vertex.1.clone())))
        })
    }
}

pub(super) fn resolve_winner_player_edge<'a, V: AsVertex<Vertex> + 'a>(
    _adapter: Arc<AdapterInner<impl ClientTransport + 'a>>,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "player" => crate::adapter::edges::winner_player::player(contexts),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Match'")
        }
    }
}

mod winner_player {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, VertexIterator,
    };

    use crate::adapter::Vertex;

    pub(super) fn player<'a, V>(
        contexts: ContextIterator<'a, V>,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>>
    where
        V: AsVertex<Vertex> + 'a,
    {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_winner_player()
                .expect("conversion failed, vertex was not a WinnerTeam");
            Box::new(std::iter::once(Vertex::Player(vertex.1.clone())))
        })
    }
}

fn resolve_winner<'a>(
    winner: &Option<Winner>,
    adapter: Arc<AdapterInner<impl ClientTransport>>,
) -> VertexIterator<'a, Vertex> {
    match winner {
        Some(Winner::Player { id: Some(id), .. }) => Box::new(
            adapter
                .execute(all::players::GetPlayer::from(*id))
                .map(|player| Vertex::WinnerPlayer { player, id: *id })
                .into_iter(),
        ),
        Some(Winner::Team { id: Some(id), .. }) => Box::new(
            adapter
                .execute(all::teams::GetTeam::from(*id))
                .map(|team| Vertex::WinnerTeam { team, id: *id })
                .into_iter(),
        ),
        _ => Box::new(std::iter::empty()),
    }
}

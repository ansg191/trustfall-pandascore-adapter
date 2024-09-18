use std::sync::Arc;

use pandascore::{
    endpoint::{all, lol, CollectionOptions},
    ClientTransport,
};
use trustfall::provider::VertexIterator;

use super::vertex::Vertex;
use crate::{
    adapter::{error::AdapterError, AdapterInner},
    pagination::PaginationIterator,
};

pub(super) fn leagues<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
    game: Option<&str>,
    search: Option<&str>,
) -> VertexIterator<'a, Vertex> {
    let mut init = CollectionOptions::new();
    if let Some(search) = search {
        init = init.search("name", search);
    }

    match game {
        Some("lol") => Box::new(
            PaginationIterator::new(adapter, lol::leagues::ListLeagues(init)).map(Vertex::League),
        ),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            Box::new(std::iter::empty())
        }
        None => Box::new(
            PaginationIterator::new(adapter, all::leagues::ListLeagues(init)).map(Vertex::League),
        ),
    }
}

pub(super) fn series<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
    game: Option<&str>,
    search: Option<&str>,
) -> VertexIterator<'a, Vertex> {
    let mut init = CollectionOptions::new();
    if let Some(search) = search {
        init = init.search("name", search);
    }

    match game {
        Some("lol") => Box::new(
            PaginationIterator::new(
                adapter,
                lol::series::ListSeries::builder().options(init).build(),
            )
            .map(Vertex::Series),
        ),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            Box::new(std::iter::empty())
        }
        None => Box::new(
            PaginationIterator::new(
                adapter,
                all::series::ListSeries::builder().options(init).build(),
            )
            .map(Vertex::Series),
        ),
    }
}

pub(super) fn tournaments<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
    game: Option<&str>,
    search: Option<&str>,
) -> VertexIterator<'a, Vertex> {
    let mut init = CollectionOptions::new();
    if let Some(search) = search {
        init = init.search("name", search);
    }

    match game {
        Some("lol") => Box::new(
            PaginationIterator::new(
                adapter,
                lol::tournaments::ListTournaments::builder()
                    .options(init)
                    .build(),
            )
            .map(|x| Vertex::Tournament(Box::new(x))),
        ),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            Box::new(std::iter::empty())
        }
        None => Box::new(
            PaginationIterator::new(
                adapter,
                all::tournament::ListTournaments::builder()
                    .options(init)
                    .build(),
            )
            .map(|x| Vertex::Tournament(Box::new(x))),
        ),
    }
}

pub(super) fn matches<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
    game: Option<&str>,
    search: Option<&str>,
) -> VertexIterator<'a, Vertex> {
    let mut init = CollectionOptions::new();
    if let Some(search) = search {
        init = init.search("name", search);
    }

    match game {
        Some("lol") => Box::new(
            PaginationIterator::new(
                adapter,
                lol::matches::ListMatches::builder().options(init).build(),
            )
            .map(|x| Vertex::Match(Box::new(x))),
        ),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            Box::new(std::iter::empty())
        }
        None => Box::new(
            PaginationIterator::new(
                adapter,
                all::matches::ListMatches::builder().options(init).build(),
            )
            .map(|x| Vertex::Match(Box::new(x))),
        ),
    }
}

pub(super) fn teams<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
    game: Option<&str>,
    search: Option<&str>,
) -> VertexIterator<'a, Vertex> {
    let mut init = CollectionOptions::new();
    if let Some(search) = search {
        init = init.search("name", search);
    }

    match game {
        Some("lol") => Box::new(
            PaginationIterator::new(adapter, lol::teams::ListTeams(init)).map(Vertex::Team),
        ),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            Box::new(std::iter::empty())
        }
        None => Box::new(
            PaginationIterator::new(adapter, all::teams::ListTeams(init)).map(Vertex::Team),
        ),
    }
}

pub(super) fn players<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
    game: Option<&str>,
    search: Option<&str>,
) -> VertexIterator<'a, Vertex> {
    let mut init = CollectionOptions::new();
    if let Some(search) = search {
        init = init.search("name", search);
    }

    match game {
        Some("lol") => Box::new(
            PaginationIterator::new(adapter, lol::players::ListPlayers(init)).map(Vertex::Player),
        ),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            Box::new(std::iter::empty())
        }
        None => Box::new(
            PaginationIterator::new(adapter, all::players::ListPlayers(init)).map(Vertex::Player),
        ),
    }
}

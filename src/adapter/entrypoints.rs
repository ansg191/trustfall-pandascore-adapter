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
) -> VertexIterator<'a, Vertex> {
    let leagues = match game {
        Some("lol") => Some(lol_leagues(adapter)),
        Some(g) => {
            adapter
                .errors()
                .push(AdapterError::InvalidGame(g.to_string()));
            None
        }
        None => Some(all_leagues(adapter)),
    };

    let Some(leagues) = leagues else {
        return Box::new(std::iter::empty());
    };

    leagues
}

fn all_leagues<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
) -> VertexIterator<'a, Vertex> {
    Box::new(
        PaginationIterator::new(adapter, CollectionOptions::default(), |adapter, next| {
            adapter.execute(all::leagues::ListLeagues(next))
        })
        .map(Vertex::League),
    )
}

fn lol_leagues<'a, T: ClientTransport + 'a>(
    adapter: Arc<AdapterInner<T>>,
) -> VertexIterator<'a, Vertex> {
    Box::new(
        PaginationIterator::new(adapter, CollectionOptions::default(), |adapter, next| {
            adapter.execute(lol::leagues::ListLeagues(next))
        })
        .map(Vertex::League),
    )
}

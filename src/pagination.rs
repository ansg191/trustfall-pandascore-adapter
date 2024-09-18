use std::{collections::VecDeque, sync::Arc};

use pandascore::{
    endpoint::{ListResponse, PaginatedEndpoint},
    ClientTransport,
};

use crate::adapter::AdapterInner;

#[derive(Debug)]
pub struct PaginationIterator<C, E: PaginatedEndpoint> {
    adapter: Arc<AdapterInner<C>>,
    init: Option<E>,
    results: VecDeque<E::Item>,
}

impl<C, E> PaginationIterator<C, E>
where
    C: ClientTransport,
    E: PaginatedEndpoint,
{
    pub fn new(adapter: Arc<AdapterInner<C>>, init: E) -> Self {
        Self {
            adapter,
            init: Some(init),
            results: VecDeque::new(),
        }
    }
}

impl<C, E, T> Iterator for PaginationIterator<C, E>
where
    C: ClientTransport,
    E: PaginatedEndpoint<Item = T, Response = ListResponse<T>> + Clone + std::fmt::Debug,
{
    type Item = E::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.results.pop_front() {
            return Some(next);
        }

        if let Some(req) = self.init.take() {
            let response = self.adapter.execute(req.clone())?;

            self.init = response.next.map(|opts| req.with_options(opts));
            self.results = response.results.into();
            self.results.pop_front()
        } else {
            None
        }
    }
}

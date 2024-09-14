use std::{collections::VecDeque, sync::Arc};

use pandascore::{
    endpoint::{CollectionOptions, ListResponse},
    ClientTransport,
};

use crate::adapter::AdapterInner;

#[derive(Debug)]
pub struct PaginationIterator<C, T, F> {
    adapter: Arc<AdapterInner<C>>,
    results: VecDeque<T>,
    next: Option<CollectionOptions>,
    f: F,
}

impl<C, T, F> PaginationIterator<C, T, F>
where
    C: ClientTransport,
    F: Fn(Arc<AdapterInner<C>>, CollectionOptions) -> Option<ListResponse<T>>,
{
    pub fn new(adapter: Arc<AdapterInner<C>>, next: CollectionOptions, f: F) -> Self {
        Self {
            adapter,
            results: VecDeque::new(),
            next: Some(next),
            f,
        }
    }
}

impl<C, T, F> Iterator for PaginationIterator<C, T, F>
where
    C: ClientTransport,
    F: Fn(Arc<AdapterInner<C>>, CollectionOptions) -> Option<ListResponse<T>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.results.pop_front() {
            return Some(next);
        }

        if let Some(next) = self.next.take() {
            let mut response = (self.f)(Arc::clone(&self.adapter), next)?;
            self.next = response.next.take();
            self.results = response.results.into();
            self.results.pop_front()
        } else {
            None
        }
    }
}

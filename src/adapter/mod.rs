mod adapter_impl;
mod edges;
mod entrypoints;
mod properties;
mod vertex;

mod error;
#[cfg(test)]
mod tests;

use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

pub use error::AdapterError;
use pandascore::{Client, ClientTransport};
use tokio::runtime::Runtime;
use trustfall::Schema;
pub use vertex::Vertex;

#[non_exhaustive]
#[derive(Debug)]
pub struct Adapter<T>(Arc<AdapterInner<T>>);

#[derive(Debug)]
pub struct AdapterInner<T> {
    runtime: Runtime,
    client: Client<T>,

    errors: Mutex<Vec<AdapterError>>,
}

static SCHEMA: OnceLock<Schema> = OnceLock::new();

impl<T: ClientTransport> Adapter<T> {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA.get_or_init(|| Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema"))
    }

    pub fn new(client: Client<T>) -> Self {
        Self(Arc::new(AdapterInner {
            runtime: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
            client,
            errors: Mutex::new(Vec::new()),
        }))
    }

    pub fn errors(&self) -> MutexGuard<Vec<AdapterError>> {
        self.0.errors()
    }
}

impl<T: ClientTransport> AdapterInner<T> {
    pub fn errors(&self) -> MutexGuard<Vec<AdapterError>> {
        self.errors.lock().unwrap()
    }

    pub(super) fn execute<R>(&self, request: R) -> Option<R::Response>
    where
        R: pandascore::endpoint::Endpoint + std::fmt::Debug,
    {
        #[cfg(feature = "log")]
        {
            log::debug!("Executing request: {request:?}");
        }

        let res = self.runtime.block_on(self.client.execute(request));
        match res {
            Ok(r) => Some(r),
            Err(e) => {
                self.errors
                    .lock()
                    .unwrap()
                    .push(AdapterError::EndpointError(e));
                None
            }
        }
    }
}

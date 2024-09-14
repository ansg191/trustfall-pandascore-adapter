use pandascore::Client;
use trustfall::provider::check_adapter_invariants;

use crate::adapter::Adapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let adapter = Adapter::new(Client::new(reqwest::Client::new(), "token").unwrap());
    let schema = Adapter::<reqwest::Client>::schema();
    check_adapter_invariants(schema, adapter);
}

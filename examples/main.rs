use std::{collections::BTreeMap, sync::Arc};

use anyhow::Context;
use pandascore::Client;
use trustfall::{execute_query, FieldValue};
use trustfall_pandascore_adapter::Adapter;

const QUERY: &str = include_str!("query.graphql");

fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let max_results = std::env::args()
        .nth(1)
        .unwrap_or("1".to_owned())
        .parse::<usize>()?;
    let args = std::env::args().skip(2).collect::<Vec<String>>();

    let client = Client::new(reqwest::Client::new(), token)?;
    let adapter = Arc::new(Adapter::new(client));

    let res = execute_query(
        Adapter::<reqwest::Client>::schema(),
        Arc::clone(&adapter),
        QUERY,
        parse_args(args),
    )?;

    if !adapter.errors().is_empty() {
        for error in adapter.errors().iter() {
            eprintln!("{}", error);
        }
    }

    for (index, data_item) in res.enumerate() {
        if !adapter.errors().is_empty() {
            for error in adapter.errors().iter() {
                eprintln!("{}", error);
            }
            break;
        }

        println!("{:#?}", data_item);

        if index + 1 >= max_results {
            break;
        }
    }

    if !adapter.errors().is_empty() {
        for error in adapter.errors().iter() {
            eprintln!("{}", error);
        }
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> BTreeMap<String, FieldValue> {
    let mut map = BTreeMap::new();

    let iter = args.chunks_exact(2);
    for chunk in iter {
        let left = &chunk[0];
        let right = &chunk[1];
        // Check if the right value is a number
        let right = if let Ok(num) = right.parse::<i64>() {
            FieldValue::Int64(num)
        } else if let Ok(num) = right.parse::<f64>() {
            FieldValue::Float64(num)
        } else if let Ok(b) = right.parse::<bool>() {
            FieldValue::Boolean(b)
        } else {
            FieldValue::String(right.as_str().into())
        };

        map.insert(left.clone(), right);
    }

    map
}

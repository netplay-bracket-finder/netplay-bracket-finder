use miette::{Diagnostic, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use tracing::{event, instrument, Level};

const API_URL: &str = "https://api.smash.gg/gql/alpha";

#[derive(Error, Debug, Diagnostic)]
#[allow(dead_code)]
enum Error {
    #[error("failed to load environment variable: {0:?}")]
    MissingVariables(envy::Error),

    #[error("request to API failed: {0:?}")]
    RequestFailed(ureq::Error),

    #[error("parsing failed: {0:?}")]
    ParseFailed(serde_json::Error),

    #[error("unknown error")]
    Unknown,
}

#[derive(Deserialize, Debug)]
struct Config {
    graphql_api_token: String,
}

pub mod api {
    //! generated from https://transform.tools/json-to-rust-serde
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
        pub data: Data,
        pub action_records: Vec<Value>,
    }

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        pub tournaments: Tournaments,
    }

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tournaments {
        pub nodes: Vec<Node>,
    }

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Node {
        pub name: String,
        pub slug: String,
        pub images: Vec<Image>,
        pub events: Vec<Event>,
    }

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Image {
        pub url: String,
        pub id: String,
    }

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Event {
        pub start_at: i64,
        pub name: String,
        pub slug: String,
        pub num_entrants: Option<i64>,
        pub images: Vec<Value>,
    }
}

#[derive(Debug, Serialize)]
pub struct TournamentEvent {
    pub slug: String,
    pub tournament_name: String,
    pub event_name: String,
    pub image: String,
    pub entrants: Option<i64>,
    #[serde(alias = "startTime")]
    pub start_time: i64,
}

#[instrument]
fn load_config() -> Result<Config> {
    match envy::from_env::<Config>() {
        Ok(config) => {
            event!(Level::INFO, "got config");
            Ok(config)
        }
        Err(e) => Err(Error::MissingVariables(e))?,
    }
}

#[instrument(skip_all)]
fn query_api(token: &str, query: &str) -> Result<ureq::Response> {
    let response = ureq::post(API_URL)
        .set("Authorization", format!("Bearer {token}").as_str())
        .send_json(ureq::json!({
            // skipping operationName + variables
            "query": query,
        }));

    event!(Level::INFO, ?response);

    match response {
        Ok(response) => Ok(response),
        Err(e) => Err(Error::RequestFailed(e))?,
    }
}

/// Convert API JSON to Elm JSON.
fn smashgg_to_elm_json(node: &api::Node) -> impl Iterator<Item = TournamentEvent> + '_ {
    node.events.iter().map(|event| TournamentEvent {
        slug: event.slug.clone(),
        tournament_name: node.name.clone(),
        event_name: event.name.clone(),
        image: match node.images.last() {
            Some(image) => image.url.clone(),
            None => "".to_string(),
        },
        entrants: event.num_entrants,
        start_time: event.start_at,
    })
}

fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();

    let config = load_config()?;

    let query = include_str!("query.graphql");
    let response = query_api(&config.graphql_api_token, query);
    let response_json = response?.into_string().expect("failed");

    let tournaments = match serde_json::from_str::<api::Root>(&response_json) {
        Ok(root) => root,
        Err(e) => Err(Error::ParseFailed(e))?,
    };

    let events: Vec<TournamentEvent> = tournaments
        .data
        .tournaments
        .nodes
        .iter()
        .flat_map(smashgg_to_elm_json)
        .collect();

    let json = serde_json::to_string_pretty(&events).expect("failed to convert");
    println!("{json}");

    Ok(())
}

use miette::{self, Diagnostic};
use serde::Deserialize;
use std::io;
use thiserror::Error;
use tracing::{event, info_span, Level};

#[derive(Error, Debug, Diagnostic)]
enum Error {
    // define an error for missing variables that inherits the error type
    // of the envy crate, and then also display it
    #[error("failed to load environment variable: {0:?}")]
    MissingVariables(envy::Error),

    #[error("unknown error")]
    Unknown,
}

#[derive(Deserialize, Debug)]
struct Config {
    graphql_api_token: String,
}

fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    // TODO: load graphql API token
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => Err(Error::MissingVariables(e))?,
    };

    let query = include_str!("query.graphql");

    let token = config.graphql_api_token;
    let resp = ureq::post("https://api.smash.gg/gql/alpha")
        .set("Authorization", format!("Bearer {token}").as_str())
        .send_json(ureq::json!({
            // skipping operationName + variables
            "query": query,
        }))
        .expect("failed to fetch");

    dbg!(resp.into_string());

    // TODO: parse JSON
    // TODO: download images
    // TODO: record tournament info to database
    // TODO: output JSON for elm
    // TODO: upload JSON to google compute bucket

    println!("hello world");

    info_span!("init").in_scope(|| {
        event!(Level::INFO, greeting = "hello world");
    });

    Ok(())
}

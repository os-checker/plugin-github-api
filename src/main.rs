#[macro_use]
extern crate tracing;

use plugin::{logger, prelude::*, repos};

mod client;
mod info;
mod workflows;

const BASE_DIR: &str = "github-api";

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let list = read_list()?;

    workflows::query(&list).await?;
    info::query(&list).await?;

    Ok(())
}

// FIXME: move this function to plugin crate
// returns a list of [user, repo]
fn read_list() -> Result<Vec<[String; 2]>> {
    repos()?
        .iter()
        .map(|s| {
            let (user, repo) = s
                .split_once('/')
                .with_context(|| format!("`{s}` is not in `user/repo` form."))?;
            eyre::Ok([user.to_owned(), repo.to_owned()])
        })
        .collect()
}

/// Display json when parse error occurs.
async fn parse_response<T: serde::de::DeserializeOwned>(
    response: github_v3::Response,
) -> Result<T> {
    let json: serde_json::Value = response.obj().await?;
    T::deserialize(&json).with_context(|| format!("json={json:#?}"))
}

#![allow(unused)]
use crate::{client::github, parse_response, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Info {
    /// repo name
    name: String,
    /// user/repo
    full_name: String,
    owner: Owner,
    description: Option<String>,
    created_at: Timestamp,
    pushed_at: Timestamp,
    /// updated_at can be influenced by many other stuff, internally or externally (like someone
    /// stars the repo), so this field is less important than pushed_at
    ///
    /// https://stackoverflow.com/questions/15918588/github-api-v3-what-is-the-difference-between-pushed-at-and-updated-at
    ///
    /// Seems this related to events.
    updated_at: Timestamp,
    homepage: Option<String>,
    default_branch: String,
    /// in bytes
    size: usize,

    /// watchers, watchers_count, and stargazers_count
    /// correspond to the number of users that have starred a repository
    ///
    /// see: https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28
    stargazers_count: usize,
    subscribers_count: usize,
    forks_count: usize,
    network_count: usize,
    open_issues_count: usize,

    fork: bool,
    archived: bool,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    has_discussions: bool,

    topics: Vec<String>,
    language: String,
    license: Option<License>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Owner {
    /// user name
    login: String,
    /// Organization, User, Bot or something
    r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct License {
    spdx_id: String,
}

async fn get_repo_info(user: &str, repo: &str) -> Result<Info> {
    let response = github().path("repos").arg(user).arg(repo).send().await?;
    parse_response(response).await
}

#[tokio::test]
async fn query() -> Result<()> {
    // let (user, repo) = ("os-checker", "os-checker");
    // let (user, repo) = ("arceos-org", "arceos");
    let (user, repo) = ("kern-crates", "sparreal-os");
    let info = get_repo_info(user, repo).await?;
    let contributors = get_repo_contributors(user, repo).await?;
    dbg!(info, contributors);
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Contributor {
    login: String,
    r#type: String,
    contributions: usize,
}

async fn get_repo_contributors(user: &str, repo: &str) -> Result<Vec<Contributor>> {
    let response = github()
        .path("repos")
        .arg(user)
        .arg(repo)
        .path("contributors")
        .send()
        .await?;
    parse_response(response).await
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    info: Info,
    contributors: Vec<Contributor>,
}

pub async fn query(user: &str, repo: &str) -> Result<Output> {
    let info = get_repo_info(user, repo).await?;
    let contributors = get_repo_contributors(user, repo).await?;
    Ok(Output { info, contributors })
}

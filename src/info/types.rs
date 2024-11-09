use crate::Result;
use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Info {
    /// repo name
    name: String,
    /// user/repo
    full_name: String,
    owner: Owner,
    description: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    pushed_at: Timestamp,
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
    license: License,
}

#[derive(Debug, Deserialize)]
struct Owner {
    /// user name
    login: String,
    /// Organization or something
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct License {
    spdx_id: String,
}

async fn get_repo_info() -> Result<Info> {
    let response = crate::client::github()
        .path("repos")
        .arg("os-checker")
        .arg("os-checker")
        .send()
        .await?;
    crate::parse_response(response).await
}

#[tokio::test]
async fn os_checker() -> Result<()> {
    let info = get_repo_info().await?;
    dbg!(info);
    Ok(())
}

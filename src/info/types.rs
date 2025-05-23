#![allow(unused)]
use super::INFO;
use crate::{client::github, parse_response, BASE_DIR};
use plugin::prelude::*;
use std::cmp::Ordering;
use tracing::Instrument;

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
    language: Option<String>,
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
async fn do_query() -> Result<()> {
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
    user: String,
    repo: String,
    info: Info,
    active_days: usize,
    contributions: usize,
    contributors: Vec<Contributor>,
}

impl Output {
    pub fn to_json(&self) -> Result<()> {
        let mut path = Utf8PathBuf::from_iter([BASE_DIR, INFO, &self.user]);
        std::fs::create_dir_all(&path)?;

        path.push(&self.repo);
        path.set_extension("json");

        let writer = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(writer, self)?;

        Ok(())
    }
}

pub async fn query(user: &str, repo: &str) -> Result<Output> {
    let span = error_span!("Info", user, repo);

    let (info, contributors) = async move {
        let info = get_repo_info(user, repo).await?;
        let contributors = get_repo_contributors(user, repo).await?;
        eyre::Ok((info, contributors))
    }
    .instrument(span.clone())
    .await?;

    let _span = span.entered();
    let active_days = active_days(info.created_at, info.pushed_at, info.updated_at);
    let contributions = contributors.iter().map(|c| c.contributions).sum();
    info!(
        active_days,
        contributions,
        contributors = contributors.len()
    );

    Ok(Output {
        user: user.to_owned(),
        repo: repo.to_owned(),
        active_days,
        contributions,
        info,
        contributors,
    })
}

fn active_days(created: Timestamp, push: Timestamp, updated: Timestamp) -> usize {
    let duration = if push >= created {
        push.duration_since(created).as_hours()
    } else if updated >= created {
        // a forked repo may have created_at time later than pushed_at
        updated.duration_since(created).as_hours()
    } else {
        0
    };
    duration as usize / 24
}

pub fn to_json(summaries: &[Output]) -> Result<()> {
    let path = Utf8PathBuf::from_iter([BASE_DIR, INFO, "summaries.json"]);

    let writer = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(writer, summaries)?;

    Ok(())
}

pub fn cmp(a: &Output, b: &Output) -> Ordering {
    // (b.contributions, &*a.user, &*a.user).cmp(&(a.contributions, &*b.user, &*b.user))
    (b.info.pushed_at, &*a.user, &*a.user).cmp(&(a.info.pushed_at, &*b.user, &*b.user))
}

use eyre::Result;
use octorust::{auth::Credentials, http_cache::HttpCache, types::WorkflowRunStatus, Client};

#[macro_use]
extern crate tracing;

mod logger;

mod time;
use time::UTC8;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let github = github_client()?;
    let actions = github.actions();

    let (user, repo) = ("os-checker", "os-checker");
    let _span = error_span!("actions", user, repo).entered();

    let workflows = actions.list_repo_workflows(user, repo, 0, 0).await?.body;
    dbg!(workflows.total_count);
    for workflow in &workflows.workflows {
        info!(
            workflow.name,
            workflow.url, ?workflow.created_at, %workflow.state
        );
    }

    let runs = actions
        .list_workflow_runs_for_repo(user, repo, "", "", "", WorkflowRunStatus::Noop, 0, 0, "")
        .await?
        .body;
    dbg!(runs.total_count, runs.workflow_runs.len());
    for run in &runs.workflow_runs {
        info!(
            run.event, run.name, run.status, run.head_branch, ?run.created_at,
            run.conclusion, run.html_url, ?run.head_commit
        );
    }

    Ok(())
}

pub fn github_client() -> Result<Client> {
    let cache = <dyn HttpCache>::in_dir(std::path::Path::new("tmp"));
    let token = std::env::var("GH_TOKEN")?;
    let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let req = reqwest::Client::builder().build()?.into();
    Ok(Client::custom(agent, Credentials::Token(token), req, cache))
}

pub struct ActionRuns {
    created_at: UTC8,
}

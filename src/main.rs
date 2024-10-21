use eyre::Result;
use octorust::{auth::Credentials, types::WorkflowRunStatus, Client};

#[macro_use]
extern crate tracing;

mod logger;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let token = std::env::var("GH_TOKEN")?;
    let github = Client::new(String::from("user-agent-name"), Credentials::Token(token))?;
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

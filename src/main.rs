use eyre::Result;

#[macro_use]
extern crate tracing;

mod client;
mod logger;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let (user, repo) = ("os-checker", "os-checker");
    let response = client::github()
        .path("repos")
        .arg(user)
        .arg(repo)
        .path("actions/runs")
        .send()
        .await?;

    let status = response.status();
    info!(status = status.as_str());

    let runs: types::WorkflowRuns = response.obj().await?;
    dbg!(&runs);

    Ok(())
}

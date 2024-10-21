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
    let _span = error_span!("actions", user, repo).entered();
    let response = client::github()
        .path("repos")
        .arg(user)
        .arg(repo)
        .path("actions/runs")
        .send()
        .await?;

    let status = response.status();
    info!(status = status.as_str());

    let runs: types::Runs = response.obj().await?;
    dbg!(&runs);

    dbg!(runs.workflow_runs[0].jobs().await?);

    Ok(())
}

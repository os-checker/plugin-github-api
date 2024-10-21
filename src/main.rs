use eyre::Result;

#[macro_use]
extern crate tracing;

mod client;
mod logger;
mod output;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let (user, repo) = ("os-checker", "os-checker");
    let wf = output::Workflows::new(user, repo).await?;
    wf.to_json()?;

    Ok(())
}

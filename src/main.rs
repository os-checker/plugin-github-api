use eyre::Result;
use octorust::{auth::Credentials, Client};

mod logger;

fn main() -> Result<()> {
    logger::init();

    let token = std::env::var("GH_TOKEN")?;
    let github = Client::new(String::from("user-agent-name"), Credentials::Token(token))?;

    Ok(())
}

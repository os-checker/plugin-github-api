#[macro_use]
extern crate tracing;

use eyre::{Context, ContextCompat, Result};
use futures::{stream::FuturesUnordered, TryStreamExt};

mod client;
mod logger;
mod output;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let args: Vec<_> = std::env::args().collect();
    // os-checker-plugin-github-api list.json
    // if the first argument (json path) is not given, it defaults to list.json.
    let list_json = args.get(1).map(|s| &**s).unwrap_or("list.json");

    let list = read_list(list_json.into())?;

    let unordered: FuturesUnordered<_> = list
        .iter()
        .map(|[user, repo]| async {
            let wf = output::Workflows::new(user, repo).await?;
            wf.to_json()?;
            eyre::Ok(())
        })
        .collect();
    unordered.try_for_each(|_| async { eyre::Ok(()) }).await?;

    Ok(())
}

// returns a list of [user, repo]
fn read_list(path: &camino::Utf8Path) -> Result<Vec<[String; 2]>> {
    let _span = error_span!("read_list", ?path).entered();
    let bytes = std::fs::read(path)?;
    serde_json::from_reader::<_, Vec<String>>(&bytes[..])
        .with_context(|| "Expected a list of string `user/repo`.")?
        .iter()
        .map(|s| {
            let (user, repo) = s
                .split_once('/')
                .with_context(|| format!("`{s}` is not in `user/repo` form."))?;
            eyre::Ok([user.to_owned(), repo.to_owned()])
        })
        .collect()
}

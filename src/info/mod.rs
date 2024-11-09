mod types;

const INFO: &str = "info";

use crate::Result;
use futures::{stream, StreamExt, TryStreamExt};

pub async fn query(list: &[[String; 2]]) -> Result<()> {
    let mut summaries: Vec<_> = stream::iter(list)
        .map(|[user, repo]| async {
            let output = types::query(user, repo).await?;
            output.to_json()?;

            eyre::Ok(output)
        })
        .buffered(10) // limit to 4 concurrent repo requests
        .try_collect()
        .await?;

    summaries.sort_unstable_by(types::cmp);
    types::to_json(&summaries)?;

    Ok(())
}

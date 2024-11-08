mod types;

mod output;
pub use output::Workflows;

mod summary;
pub use summary::{to_json, Summary};

use crate::Result;
use futures::{stream, StreamExt, TryStreamExt};

pub async fn query(list: &[[String; 2]]) -> Result<()> {
    let mut summaries: Vec<_> = stream::iter(list)
        .map(|[user, repo]| async {
            let wf = Workflows::new(user, repo).await?;
            wf.to_json()?;

            eyre::Ok(Summary::new(&wf))
        })
        .buffered(10) // limit to 4 concurrent repo requests
        .try_collect()
        .await?;

    summaries.sort_unstable_by(Summary::cmp_by_timestamp);
    to_json(&summaries)?;

    Ok(())
}

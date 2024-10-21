use crate::{types::*, Result};
use futures::{stream::FuturesOrdered, TryStreamExt};
use serde::Serialize;
use tracing::Instrument;

#[derive(Debug, Serialize)]
pub struct Workflow {
    run: Run,
    jobs: Jobs,
}

impl Workflow {
    /// fill in missing fields which are computed after deserialization
    pub fn check(&mut self) {
        self.run.check();
        self.jobs.check();
    }
}

#[derive(Debug, Serialize)]
pub struct Workflows {
    user: String,
    repo: String,
    runs_total_count: usize,
    workflows: Vec<Workflow>,
}

impl Workflows {
    async fn workflows(runs: Runs) -> Result<Vec<Workflow>> {
        let ordered: FuturesOrdered<_> = runs
            .workflow_runs
            .into_iter()
            .map(|run| async {
                let jobs = run.jobs().await?;
                let mut workflow = Workflow { run, jobs };
                workflow.check();
                eyre::Ok(workflow)
            })
            .collect();
        ordered.try_collect().await
    }

    pub async fn new(user: &str, repo: &str) -> Result<Self> {
        let span = error_span!("Workflows", user, repo);

        let (runs_total_count, workflows) = async move {
            let response = crate::client::github()
                .path("repos")
                .arg(user)
                .arg(repo)
                .path("actions/runs")
                .send()
                .await?;

            let runs: Runs = response.obj().await?;
            let runs_total_count = runs.total_count;
            let workflows = Self::workflows(runs).await?;
            eyre::Ok((runs_total_count, workflows))
        }
        .instrument(span.clone())
        .await?;

        let _span = span.entered();
        info!(workflows.len = workflows.len());

        Ok(Workflows {
            user: user.to_owned(),
            repo: repo.to_owned(),
            runs_total_count,
            workflows,
        })
    }

    pub fn to_json(&self) -> Result<()> {
        let base = "tmp";
        let mut path = camino::Utf8PathBuf::from_iter([base, &self.user]);
        std::fs::create_dir_all(&path)?;

        path.push(&self.repo);
        path.set_extension("json");

        let writer = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(writer, self)?;

        Ok(())
    }
}

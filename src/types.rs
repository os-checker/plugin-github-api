use crate::{client::github, Result};
use eyre::Context;
use github_v3::Builder;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Runs {
    pub total_count: usize,
    pub workflow_runs: Vec<Run>,
}

#[derive(Debug, Deserialize)]
pub struct Actor {
    login: String,
}

#[derive(Debug, Deserialize)]
pub struct Run {
    pub name: String,
    pub head_branch: String,
    pub head_sha: String,
    #[serde(rename(deserialize = "display_title"))]
    pub title: String,
    pub html_url: String,
    pub event: String,
    pub status: String,
    pub conclusion: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    // "pull_requests": [], https://docs.rs/octorust/latest/octorust/types/struct.WorkflowRun.html#structfield.pull_requests
    pub actor: Actor,
    pub triggering_actor: Actor,
    pub id: usize,
    pub jobs_url: String,
    pub logs_url: String,
}

const PREFIX: &str = "https://api.github.com/";

impl Run {
    pub fn req_jobs(&self) -> Builder {
        let path = self.jobs_url.strip_prefix(PREFIX).unwrap();
        let mut client = github();
        for arg in path.split("/") {
            client = client.arg(arg);
        }
        client
    }

    pub async fn jobs(&self) -> Result<Jobs> {
        let _span = error_span!("Jobs", self.name, self.id).entered();
        let response = self.req_jobs().send().await?;
        response.obj().await.with_context(|| "Failed to get jobs.")
    }
}

#[derive(Debug, Deserialize)]
pub struct Jobs {
    pub total_count: usize,
    pub jobs: Vec<Job>,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub workflow_name: String,
    pub head_branch: String,
    pub html_url: String,
    pub status: String,
    pub conclusion: String,
    pub created_at: Timestamp,
    pub started_at: Timestamp,
    pub completed_at: Timestamp,
    pub steps: Vec<Step>,
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub name: String,
    pub status: String,
    pub conclusion: String,
    pub number: usize,
    pub started_at: Timestamp,
    pub completed_at: Timestamp,
}

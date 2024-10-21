#![allow(dead_code)]

use crate::{client::github, Result};
use eyre::Context;
use github_v3::Builder;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Instrument;

#[derive(Debug, Deserialize, Serialize)]
pub struct Runs {
    pub total_count: usize,
    pub workflow_runs: Vec<Run>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    pub login: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Run {
    pub name: String,
    pub head_branch: String,
    pub head_sha: String,
    pub head_commit: HeadCommit,
    pub display_title: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct HeadCommit {
    pub message: String,
    pub timestamp: Timestamp,
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
        let span = error_span!("Jobs", self.name, self.id);
        async move {
            let response = self.req_jobs().send().await?;
            response.obj().await.with_context(|| "Failed to get jobs.")
        }
        .instrument(span)
        .await
    }

    pub fn duration_sec(&self) -> i64 {
        duration_sec(self.created_at, self.updated_at)
    }
}

fn duration_sec(earlier: Timestamp, later: Timestamp) -> i64 {
    later.duration_since(earlier).as_secs()
}

// https://docs.github.com/en/rest/actions/workflow-runs?apiVersion=2022-11-28#download-workflow-run-logs
// https://api.github.com/repos/OWNER/REPO/actions/runs/RUN_ID/logs => logs in .zip

#[derive(Debug, Deserialize, Serialize)]
pub struct Jobs {
    pub total_count: usize,
    pub jobs: Vec<Job>,
}

#[derive(Debug, Deserialize, Serialize)]
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

impl Job {
    pub fn duration_sec(&self) -> i64 {
        duration_sec(self.started_at, self.completed_at)
    }
}

// https://docs.github.com/en/rest/actions/workflow-jobs?apiVersion=2022-11-28#download-job-logs-for-a-workflow-run
// https://api.github.com/repos/OWNER/REPO/actions/jobs/JOB_ID/logs => single txt

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    pub name: String,
    pub status: String,
    pub conclusion: String,
    pub number: usize,
    pub started_at: Timestamp,
    pub completed_at: Timestamp,
}

impl Step {
    pub fn duration_sec(&self) -> i64 {
        duration_sec(self.started_at, self.completed_at)
    }
}

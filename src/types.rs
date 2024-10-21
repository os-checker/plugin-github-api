use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowRuns {
    pub total_count: usize,
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowRun {
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
}

#[derive(Debug, Deserialize)]
pub struct Actor {
    login: String,
}

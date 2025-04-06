use super::{
    output::{Workflow, Workflows},
    types::{duration_sec, HeadCommit},
    WORKFLOWS,
};
use crate::BASE_DIR;
use plugin::prelude::*;
use std::cmp::Ordering;

/// Latest workflow is the latest updated (first) & latest created (second) workflow.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LastWorkflow {
    /// Find the created_at timestamp of latest workflow.
    pub created_at: Timestamp,
    /// Find the updated_at timestamp of latest workflow.
    pub updated_at: Timestamp,
    /// A substraction from above timestamps: possible incorrect in many ways though.
    pub duration_sec: i64,
    pub completed: bool,
    pub success: bool,
    pub head_branch: String,
    pub head_sha: String,
    pub head_commit: HeadCommit,
    pub workflows: Vec<Workflow>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Summary {
    pub user: String,
    pub repo: String,
    // total count of workflow runs
    pub runs: usize,
    pub last: Option<LastWorkflow>,
}

impl Summary {
    pub fn new(workflows: &Workflows) -> Summary {
        let mut summary = Summary {
            user: workflows.user.clone(),
            repo: workflows.repo.clone(),
            runs: workflows.runs_total_count,
            last: None,
        };

        if workflows.runs_total_count == 0 {
            return summary;
        }

        // group b head_sha
        let wfs = &workflows.workflows;
        let mut groups = IndexMap::<&str, Vec<&Workflow>>::with_capacity(wfs.len());
        for wf in wfs {
            let sha = wf.run.head_sha.as_str();
            groups
                .entry(sha)
                .and_modify(|v| v.push(wf))
                .or_insert_with(|| vec![wf]);
        }

        groups.sort_unstable_by(|_, a, _, b| {
            let a_updated = a.iter().map(|a| a.run.updated_at).max().unwrap();
            let a_created = a.iter().map(|a| a.run.created_at).max().unwrap();
            let b_updated = b.iter().map(|b| b.run.updated_at).max().unwrap();
            let b_created = b.iter().map(|b| b.run.created_at).max().unwrap();
            (b_updated, b_created).cmp(&(a_updated, a_created))
        });
        // index 0 means the latest workflow in each group
        let wf = &groups[0];

        let run = &wf[0].run;
        let created_at = run.created_at;
        let updated_at = run.updated_at;

        // only keep the first run name because repeated run names exist for the
        // same sha due to scheduling
        let unique_name = {
            let mut v = wf.to_vec();
            v.sort_by_key(|w| w.run.name.as_str());
            v.dedup_by_key(|w| w.run.name.as_str());
            v
        };
        summary.last = Some(LastWorkflow {
            created_at,
            updated_at,
            duration_sec: duration_sec(created_at, updated_at),
            // must check all latset data
            completed: unique_name.iter().all(|w| w.run.status == "completed"),
            success: unique_name
                .iter()
                .all(|w| w.run.conclusion.as_deref() == Some("success")),
            head_branch: run.head_branch.clone(),
            head_sha: run.head_sha.clone(),
            head_commit: run.head_commit.clone(),
            workflows: wf.iter().map(|w| (*w).clone()).collect(),
        });
        summary
    }

    pub fn cmp_by_timestamp(&self, other: &Self) -> Ordering {
        let timestamp_a = self.last.as_ref().map(|l| (l.updated_at, l.created_at));
        let timestamp_b = other.last.as_ref().map(|l| (l.updated_at, l.created_at));
        match [timestamp_a, timestamp_b] {
            // alphabetic sort if neither has workflow runs
            [None, None] => (&self.user, &self.repo).cmp(&(&other.user, &other.repo)),
            // Either with a workflow run wins
            [None, Some(_)] => Ordering::Greater,
            [Some(_), None] => Ordering::Less,
            // latest timestamp wins
            [Some(a), Some(b)] => b.cmp(&a),
        }
    }
}

pub fn to_json(summaries: &[Summary]) -> Result<()> {
    let path = Utf8PathBuf::from_iter([BASE_DIR, WORKFLOWS, "summaries.json"]);

    let writer = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(writer, summaries)?;

    Ok(())
}

use crate::{
    output::{Workflow, Workflows},
    types::{duration_sec, HeadCommit},
    Result, BASE_DIR,
};
use indexmap::IndexMap;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, Reverse};

/// Latest workflow is the latest updated (first) & latest created (second) workflow.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LastWorkflow {
    /// Find the created_at timestamp of latest workflow.
    pub created_at: Timestamp,
    /// Find the updated_at timestamp of latest workflow.
    pub updated_at: Timestamp,
    /// A substraction from above timestamps: possible incorrect in many ways though.
    pub duration_sec: i64,
    pub status: String,
    pub conclusion: String,
    pub head_branch: String,
    pub head_sha: String,
    pub head_commit: HeadCommit,
    pub workflow: Workflow,
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

        // sort each workflow with latest first
        for v in groups.values_mut() {
            v.sort_by_key(|wf| Reverse((wf.run.updated_at, wf.run.created_at)));
        }
        // index 0 means the latest workflow in each group
        groups.sort_unstable_by(|_, a, _, b| {
            (b[0].run.updated_at, b[0].run.created_at)
                .cmp(&(a[0].run.updated_at, a[0].run.created_at))
        });
        let wf = &groups[0][0];

        let run = &wf.run;
        let created_at = run.created_at;
        let updated_at = run.updated_at;

        summary.last = Some(LastWorkflow {
            created_at,
            updated_at,
            duration_sec: duration_sec(created_at, updated_at),
            status: run.status.clone(),
            conclusion: run.conclusion.clone(),
            head_branch: run.head_branch.clone(),
            head_sha: run.head_sha.clone(),
            head_commit: run.head_commit.clone(),
            workflow: (*wf).clone(),
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
    let path = camino::Utf8PathBuf::from_iter([BASE_DIR, "summaries.json"]);

    let writer = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(writer, summaries)?;

    Ok(())
}
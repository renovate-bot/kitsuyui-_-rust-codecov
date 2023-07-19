use serde::{Deserialize, Serialize};

/**
 * Totals is a struct that represents the totals for a commit.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Totals {
    pub files: usize,
    pub lines: usize,
    pub hits: usize,
    pub misses: usize,
    pub partials: usize,
    pub coverage: f64,
    pub branches: usize,
    pub methods: usize,
    pub sessions: usize,
    pub complexity: f64,
    pub complexity_total: f64,
    pub complexity_ratio: f64,
    pub diff: Diff,
}

/**
 * Diff is a struct that represents the diff for a commit.
 * Diff may be a u64 or an array of Option<String> like this:
 * "diff": [0, 0, 0, 0, 0, null, 0, 0, 0, 0, null, null, 0]
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Diff {
    Value(u64),
    Array(Vec<Option<u64>>),
}

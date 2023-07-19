/**
 * Codecov v2 API
 * /repos endpoint returns a list of repos for a given owner.
 */
use serde::{Deserialize, Serialize};

use crate::url::Url;

/**
 * CommitsAPIResponse is a struct that represents the response from the commits API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitsAPIResponse {
    pub results: Vec<Commit>,
    pub count: usize,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub total_pages: usize,
}

/**
 * CommitAuthor is a struct that represents the author of a commit.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitAuthor {
    pub service: String,
    pub username: String,
    pub name: Option<String>,
}

/**
 * Commit is a struct that represents a commit.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub commitid: String,
    pub message: String,
    pub timestamp: String, // TODO: ISO Date
    pub ci_passed: bool,
    pub author: CommitAuthor,
    pub branch: Option<String>,
    pub totals: Totals,
    pub state: String,
    pub parent: Option<String>,
}

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
    pub diff: usize,
}

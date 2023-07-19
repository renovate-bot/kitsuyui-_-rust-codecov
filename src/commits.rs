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

impl CommitsAPIResponse {
    pub fn coverage(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        let mut total_coverage = 0.0;
        for commit in &self.results {
            total_coverage += commit.totals.coverage;
        }
        Some(total_coverage / self.count as f64)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_coverage() {
        use super::*;
        let mut response = CommitsAPIResponse {
            results: vec![],
            count: 1,
            next: None,
            previous: None,
            total_pages: 1,
        };
        assert_eq!(response.coverage(), Some(0.0));
        let commit = Commit {
            commitid: String::from("123"),
            message: String::from("message"),
            timestamp: String::from("timestamp"),
            ci_passed: true,
            author: CommitAuthor {
                service: String::from("service"),
                username: String::from("username"),
                name: Some(String::from("name")),
            },
            branch: Some(String::from("branch")),
            totals: Totals {
                files: 1,
                lines: 1,
                hits: 1,
                misses: 1,
                partials: 1,
                coverage: 2.0,
                branches: 1,
                methods: 1,
                sessions: 1,
                complexity: 1.0,
                complexity_total: 1.0,
                complexity_ratio: 1.0,
                diff: 1,
            },
            state: String::from("state"),
            parent: Some(String::from("parent")),
        };
        response.results.push(commit);
        assert_eq!(response.coverage(), Some(2.0));
    }
}

use serde::{Deserialize, Serialize};

use crate::author::Author;
use crate::totals::Totals;

/**
 * BranchDetailAPIResponse is an enum wrapping all possible responses from the branches API.
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BranchDetailAPIResponse {
    Success(Box<BranchDetailAPISuccessResponse>),
    NotFound(BranchNotFound),
}

/**
 * BranchesAPIResponse is a struct that represents the response from the branches API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BranchDetailAPISuccessResponse {
    pub head_commit: HeadCommit,
    pub name: String,
    pub updatestamp: String, // TODO: ISO Date
}

/**
 * BranchNotFound is a struct that represents a branch not found error.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BranchNotFound {
    pub detail: String,
}

/**
 * Branch is a struct that represents a branch.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct HeadCommit {
    pub author: Author,
    pub branch: String,
    pub ci_passed: Option<bool>,
    pub commitid: String,
    pub message: String,
    pub parent: String,
    pub report: Report,
    pub state: String,
    pub timestamp: String, // TODO: ISO Date
    pub totals: Totals,
}

/**
 * Report is a struct that represents a report.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub files: Vec<File>,
    pub totals: Totals,
}

/**
 * File is a struct that represents a file.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub totals: Totals,
}

impl BranchDetailAPIResponse {
    /**
     * Returns the latest coverage for a branch.
     */
    pub fn latest_coverage(&self) -> f64 {
        match self {
            BranchDetailAPIResponse::Success(branch_detail) => branch_detail.latest_coverage(),
            BranchDetailAPIResponse::NotFound(_) => 0.0,
        }
    }
}

impl BranchDetailAPISuccessResponse {
    /**
     * Returns the latest coverage for a branch.
     */
    pub fn latest_coverage(&self) -> f64 {
        self.head_commit.totals.coverage
    }
}

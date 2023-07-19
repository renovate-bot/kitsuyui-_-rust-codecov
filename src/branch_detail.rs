use serde::{Deserialize, Serialize};

use crate::author::Author;
use crate::totals::Totals;

/**
 * BranchesAPIResponse is a struct that represents the response from the branches API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BranchDetailAPIResponse {
    pub head_commit: HeadCommit,
    pub name: String,
    pub updatestamp: String, // TODO: ISO Date
}

/**
 * Branch is a struct that represents a branch.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct HeadCommit {
    pub author: Author,
    pub branch: String,
    pub ci_passed: bool,
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

use serde::{Deserialize, Serialize};

use crate::url::Url;

/**
 * BranchesAPIResponse is a struct that represents the response from the branches API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BranchesAPIResponse {
    pub results: Vec<Branch>,
    pub count: usize,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub total_pages: usize,
}

/**
 * Branch is a struct that represents a branch.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
    pub name: String,
    pub updatestamp: String, // TODO: ISO Date
}

/**
 * Codecov v2 API
 * /repos endpoint returns a list of repos for a given owner.
 */
use serde::{Deserialize, Serialize};

/**
 * ReposAPIResponse is a struct that represents the response from the repos API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ReposAPIResponse {
    pub results: Vec<Repo>,
    pub count: usize,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub total_pages: usize,
}

/**
 * Url is just a string that represents a URL. (currently)
 */
type Url = String;

/**
 * Repo is a struct that represents a repo.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub private: bool,
    pub updatestamp: String, // TODO: ISO Date
    pub author: Author,
    pub language: Option<String>,
    pub branch: Option<String>,
    pub active: bool,
    pub activated: bool,
}

/**
 * Author is a struct that represents the author of a repo.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub service: String,
    pub username: String,
    pub name: String,
}

use serde::{Deserialize, Serialize};
/**
 * Author is a struct that represents the author of a repo.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub service: String,
    pub username: String,
    pub name: String,
}

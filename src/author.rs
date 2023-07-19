use serde::{Deserialize, Serialize};

use crate::owner::Owner;

/**
 * Author is a struct that represents the author of a repo.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub service: String,
    pub username: String,
    pub name: String,
}

impl Author {
    pub fn new(service: &str, username: &str, name: &str) -> Author {
        Author {
            service: service.to_string(),
            username: username.to_string(),
            name: name.to_string(),
        }
    }

    pub fn from_owner(owner: &Owner, name: &str) -> Author {
        Self::new(&owner.service, &owner.username, name)
    }

    pub fn to_owner(&self) -> Owner {
        Owner::new(&self.service, &self.username)
    }
}

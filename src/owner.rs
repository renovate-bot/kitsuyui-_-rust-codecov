use crate::author::Author;
/**
 * Owner is a struct that represents an owner of repos.
 */
pub struct Owner {
    pub service: String,
    pub username: String,
}

impl Owner {
    pub fn new(service: &str, username: &str) -> Owner {
        Owner {
            service: service.to_string(),
            username: username.to_string(),
        }
    }

    /**
     * Returns a new Author for a given repo name.
     */
    pub fn new_author(&self, name: &str) -> Author {
        Author::from_owner(self, name)
    }
}

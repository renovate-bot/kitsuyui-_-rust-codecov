mod repos;

/**
 * Error is an enum wrapping all possible errors.
 */
#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    EnvError(std::env::VarError),
}

/**
 * Client is a struct that represents a client to the Codecov API.
 */
pub struct Client {
    token: String,
}

/**
 * Owner is a struct that represents an owner of repos.
 */
pub struct Owner {
    service: String,
    username: String,
}

impl Client {
    pub fn new_from_env() -> Result<Client, Error> {
        let token = match std::env::var("CODECOV_OWNER_TOKEN") {
            Ok(token) => token,
            Err(e) => return Err(Error::EnvError(e)),
        };
        Ok(Client::new(token))
    }

    pub fn new(token: String) -> Client {
        Client { token }
    }

    fn auth_header_val(&self) -> String {
        format!("bearer {}", self.token)
    }

    fn owner_endpoint(&self, owner: &Owner) -> String {
        format!(
            "{}/{}/{}",
            "https://codecov.io/api/v2", owner.service, owner.username
        )
    }

    /**
     * get_all_repos returns a list of all repos for a given owner.
     * /repos endpoint returns a list of repos for a given owner with pagination.
     * This function will make multiple requests to get all repos.
     */
    pub fn get_all_repos(&self, owner: &Owner) -> Result<Vec<repos::Repo>, Error> {
        let mut repos = Vec::new();
        let mut url = format!("{}/repos?page_size=100", self.owner_endpoint(owner));
        loop {
            let mut repo_list = self.get_repos_page(&url)?;
            match &mut repo_list {
                repos::ReposAPIResponse {
                    results,
                    next: Some(next_url),
                    ..
                } => {
                    repos.append(results);
                    url = next_url.to_string();
                    continue;
                }
                repos::ReposAPIResponse {
                    results,
                    next: None,
                    ..
                } => {
                    repos.append(results);
                    break;
                }
            }
        }
        Ok(repos)
    }

    /**
     * get_repos_page returns a single page of repos.
     * This is a helper function for get_all_repos.
     */
    fn get_repos_page(&self, url: &str) -> Result<repos::ReposAPIResponse, Error> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(url)
            .header("Authorization", self.auth_header_val());
        let res = match req.send() {
            Ok(res) => res,
            Err(e) => return Err(Error::ReqwestError(e)),
        };
        let repos = match res.json::<repos::ReposAPIResponse>() {
            Ok(repos) => repos,
            Err(e) => return Err(Error::ReqwestError(e)),
        };
        Ok(repos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_repos() {
        let client = Client::new_from_env().unwrap();
        let owner = Owner {
            service: "github".to_string(),
            username: "codecov".to_string(),
        };
        let repos = client.get_all_repos(&owner).unwrap();
        assert!(!repos.is_empty());
    }
}

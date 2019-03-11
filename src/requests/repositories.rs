use super::ToUrl;
use crate::repository::Role;

/// Request for a list of repositories
#[derive(Debug)]
pub struct RepositoriesRequest {
    /// username or team to list repositories of
    pub username: Option<String>,
    /// list repositories where the current logged in user as the following role
    pub role: Option<Role>,
}

impl From<()> for RepositoriesRequest {
    fn from(_: ()) -> Self {
        Self {
            username: None,
            role: None,
        }
    }
}

impl From<&str> for RepositoriesRequest {
    fn from(value: &str) -> Self {
        Self {
            username: Some(String::from(value)),
            role: None,
        }
    }
}

impl From<String> for RepositoriesRequest {
    fn from(value: String) -> Self {
        Self {
            username: Some(value),
            role: None,
        }
    }
}

impl From<Role> for RepositoriesRequest {
    fn from(value: Role) -> Self {
        Self {
            username: None,
            role: Some(value),
        }
    }
}

impl From<(&str, Role)> for RepositoriesRequest {
    fn from((username, role): (&str, Role)) -> Self {
        Self {
            username: Some(String::from(username)),
            role: Some(role),
        }
    }
}

impl From<(String, Role)> for RepositoriesRequest {
    fn from((username, role): (String, Role)) -> Self {
        Self {
            username: Some(username),
            role: Some(role),
        }
    }
}

impl ToUrl for RepositoriesRequest {
    fn to_url(&self, base_url: &str) -> Result<reqwest::Url, crate::error::Error> {
        let url = match &self.username {
            None => format!("{}/repositories", base_url),
            Some(username) => format!("{}/repositories/{}", base_url, username),
        };

        let mut url = reqwest::Url::parse(&url).map_err(|_| crate::error::Error::InvalidUrl {
            url: String::from(url),
        })?;

        if let Some(role) = &self.role {
            url.query_pairs_mut().append_pair(
                "role",
                match role {
                    Role::Admin => "admin",
                    Role::Contributor => "contributor",
                    Role::Member => "member",
                    Role::Owner => "owner",
                },
            );
        }

        Ok(url)
    }
}

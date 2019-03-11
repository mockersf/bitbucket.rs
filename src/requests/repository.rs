use serde::Serialize;

use super::ToUrl;

/// Request a repository by it's owner and name
#[derive(Debug)]
pub struct RepositoryRequest {
    /// owner of the repository
    pub username: String,
    /// name of the repository
    pub repo_slug: String,
}

impl From<(&str, &str)> for RepositoryRequest {
    fn from((username, repo_slug): (&str, &str)) -> Self {
        Self {
            username: String::from(username),
            repo_slug: String::from(repo_slug),
        }
    }
}

impl From<(&str, String)> for RepositoryRequest {
    fn from((username, repo_slug): (&str, String)) -> Self {
        Self {
            username: String::from(username),
            repo_slug,
        }
    }
}

impl From<(String, &str)> for RepositoryRequest {
    fn from((username, repo_slug): (String, &str)) -> Self {
        Self {
            username,
            repo_slug: String::from(repo_slug),
        }
    }
}

impl From<(String, String)> for RepositoryRequest {
    fn from((username, repo_slug): (String, String)) -> Self {
        Self {
            username,
            repo_slug,
        }
    }
}

impl ToUrl for RepositoryRequest {
    fn to_url(&self, base_url: &str) -> Result<reqwest::Url, crate::error::Error> {
        let url = format!(
            "{}/repositories/{}/{}",
            base_url, self.username, self.repo_slug
        );

        let url = reqwest::Url::parse(&url).map_err(|_| crate::error::Error::InvalidUrl { url })?;

        Ok(url)
    }
}

/// request to create a repository
#[derive(Serialize, Default, Debug, Copy, Clone)]
pub struct RepositoryCreationRequest {
    /// is the repository private
    pub is_private: Option<bool>,
    /// SCM used by the new repository
    pub scm: Option<crate::repository::Scm>,
}

impl RepositoryCreationRequest {
    /// set the privacy of the repository that will be created
    pub fn private(self, private: bool) -> Self {
        Self {
            is_private: Some(private),
            ..self
        }
    }

    /// set the SCM of the repository that will be created
    pub fn scm(self, scm: crate::repository::Scm) -> Self {
        Self {
            scm: Some(scm),
            ..self
        }
    }
}

impl From<()> for RepositoryCreationRequest {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

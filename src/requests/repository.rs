use serde::Serialize;

use super::ToUrl;

pub struct RepositoryRequest {
    pub username: String,
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
            repo_slug: repo_slug,
        }
    }
}

impl From<(String, &str)> for RepositoryRequest {
    fn from((username, repo_slug): (String, &str)) -> Self {
        Self {
            username: username,
            repo_slug: String::from(repo_slug),
        }
    }
}

impl From<(String, String)> for RepositoryRequest {
    fn from((username, repo_slug): (String, String)) -> Self {
        Self {
            username: username,
            repo_slug: repo_slug,
        }
    }
}

impl ToUrl for RepositoryRequest {
    fn to_url(&self, base_url: &str) -> Result<reqwest::Url, crate::error::Error> {
        let url = format!(
            "{}/repositories/{}/{}",
            base_url, self.username, self.repo_slug
        );

        let url = reqwest::Url::parse(&url).map_err(|_| crate::error::Error::InvalidUrl {
            url: String::from(url),
        })?;

        Ok(url)
    }
}

#[derive(Serialize, Default)]
pub struct RepositoryCreationRequest {
    is_private: Option<bool>,
    scm: Option<crate::repository::Scm>,
}

impl RepositoryCreationRequest {
    pub fn private(self, private: bool) -> Self {
        Self {
            is_private: Some(private),
            ..self
        }
    }

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

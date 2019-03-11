use crate::internal_api::AuthType;
use crate::repository::Repository;

use crate::requests::ToUrl;

pub struct API {
    url: String,
    pub(crate) auth_type: Option<AuthType>,
    pub(crate) client: Option<reqwest::Client>,
}

impl API {
    pub fn new() -> Self {
        Self {
            url: String::from("https://api.bitbucket.org/2.0"),
            auth_type: None,
            client: None,
        }
    }

    pub fn with_bearer(self, bearer: &str) -> Self {
        Self {
            auth_type: Some(AuthType::Bearer {
                token: String::from(bearer),
            }),
            ..self
        }
    }

    pub fn with_basic(self, username: &str, password: &str) -> Self {
        Self {
            auth_type: Some(AuthType::Basic {
                username: String::from(username),
                password: String::from(password),
            }),
            ..self
        }
    }

    pub fn get_repositories(
        &self,
        request: impl Into<crate::requests::RepositoriesRequest>,
    ) -> Result<crate::Paginated<Repository>, crate::Error> {
        self.get_paginated(request.into().to_url(&self.url)?)
    }

    pub fn get_repository(
        &self,
        request: impl Into<crate::requests::RepositoryRequest>,
    ) -> Result<Repository, crate::Error> {
        self.get(&request.into().to_url(&self.url)?)
    }

    pub fn create_repository(
        &self,
        request: impl Into<crate::requests::RepositoryRequest>,
        repository_creation: impl Into<crate::requests::RepositoryCreationRequest>,
    ) -> Result<Repository, crate::Error> {
        self.post(
            &request.into().to_url(&self.url)?,
            repository_creation.into(),
        )
    }
}

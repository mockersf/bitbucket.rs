use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Link {
    href: String,
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryLinks {
    pub watchers: Link,
    pub branches: Link,
    pub tags: Link,
    pub commits: Link,
    pub clone: Vec<Link>,
    #[serde(rename = "self")]
    pub zelf: Link,
    pub source: Link,
    pub html: Link,
    pub avatar: Link,
    pub hooks: Link,
    pub forks: Link,
    pub downloads: Link,
    pub pullrequests: Link,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Scm {
    Git,
    Mercurial,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ForkPolicy {
    NoPublicForks,
    NoForks,
    AllowForks,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Branch,
    Repository,
    Project,
    Team,
}

#[derive(Deserialize, Debug)]
pub struct ShortLinks {
    #[serde(rename = "self")]
    pub zelf: Link,
    pub html: Link,
    pub avatar: Link,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub key: String,
    #[serde(rename = "type")]
    ty: ObjectType,
    pub uuid: String,
    pub links: ShortLinks,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub username: String,
    pub display_name: String,
    #[serde(rename = "type")]
    ty: ObjectType,
    pub uuid: String,
    pub links: ShortLinks,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub scm: Scm,
    pub website: String,
    pub has_wiki: bool,
    pub uuid: String,
    pub links: RepositoryLinks,
    pub fork_policy: ForkPolicy,
    pub name: String,
    pub language: String,
    pub created_on: DateTime<Utc>,
    pub mainbranch: Option<Branch>,
    pub full_name: String,
    pub has_issues: bool,
    pub updated_on: DateTime<Utc>,
    pub size: u32,
    #[serde(rename = "type")]
    ty: ObjectType,
    pub slug: String,
    pub is_private: bool,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Branch {
    #[serde(rename = "type")]
    ty: ObjectType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Contributor,
    Member,
    Owner,
}

pub struct RepositoryRequest {
    pub username: Option<String>,
    pub role: Option<Role>,
}

impl From<()> for RepositoryRequest {
    fn from(_: ()) -> Self {
        Self {
            username: None,
            role: None,
        }
    }
}

impl From<&str> for RepositoryRequest {
    fn from(value: &str) -> Self {
        Self {
            username: Some(String::from(value)),
            role: None,
        }
    }
}

impl From<String> for RepositoryRequest {
    fn from(value: String) -> Self {
        Self {
            username: Some(value),
            role: None,
        }
    }
}

impl From<Role> for RepositoryRequest {
    fn from(value: Role) -> Self {
        Self {
            username: None,
            role: Some(value),
        }
    }
}

pub(crate) trait ToUrl {
    fn to_url(&self, base_url: &str) -> Result<reqwest::Url, crate::error::Error>;
}

impl ToUrl for RepositoryRequest {
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

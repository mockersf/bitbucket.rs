use chrono::prelude::*;
use serde::Deserialize;

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

mod repositories;
mod repository;

pub use repositories::RepositoriesRequest;
pub use repository::{RepositoryCreationRequest, RepositoryRequest};

pub(crate) trait ToUrl {
    fn to_url(&self, base_url: &str) -> Result<reqwest::Url, crate::error::Error>;
}

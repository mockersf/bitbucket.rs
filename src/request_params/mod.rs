mod repositories;

pub use repositories::RepositoriesRequest;

pub(crate) trait ToUrl {
    fn to_url(&self, base_url: &str) -> Result<reqwest::Url, crate::error::Error>;
}

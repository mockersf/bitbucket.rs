/// A page from a paginated response.
#[derive(Debug)]
pub struct Paginated<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    /// Are there more pages?
    pub has_more: bool,
    pub(crate) current_page: crate::internal_api::Page<T>,
    pub(crate) client: &'a crate::api::API,
}

impl<'a, T> Paginated<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    /// Get the values of the current page.
    pub fn current_page_values(self) -> Vec<T> {
        self.current_page.values
    }
}

impl<'a, T> IntoIterator for Paginated<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
    T: std::fmt::Debug,
{
    type Item = T;
    type IntoIter = PageIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.current_page.get_all_pages(self.client)
    }
}

/// Iterator over all values and across pages, built from a paginated response. Iterating
/// over it will requests new pages from Bitbucket API as needed.
#[derive(Debug)]
pub struct PageIterator<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub(crate) next_page: Option<String>,
    pub(crate) current_page: Vec<T>,
    pub(crate) api: &'a crate::API,
}

impl<'a, T> Iterator for PageIterator<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
    T: std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.current_page.pop();
        match (result, &self.next_page) {
            (Some(result), _) => Some(result),
            (None, Some(ref url)) => {
                let next = reqwest::Url::parse(url)
                    .map_err(|_| crate::error::Error::InvalidUrl { url: url.clone() })
                    .and_then(|url| self.api.get::<crate::internal_api::Page<T>>(&url));
                match next {
                    Ok(new_page) => {
                        self.next_page = new_page.next;
                        self.current_page = new_page.values;
                        self.current_page.pop()
                    }
                    _ => None,
                }
            }
            (None, None) => None,
        }
    }
}

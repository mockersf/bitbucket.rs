pub struct Paginated<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub has_more: bool,
    pub(crate) current_page: crate::internal_api::Page<T>,
    pub(crate) client: &'a crate::api::API,
}

impl<'a, T> Paginated<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
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
    type IntoIter = crate::PageIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.current_page.get_all_pages(self.client)
    }
}

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
                    .and_then(|url| self.api.get_page::<T>(&url));
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

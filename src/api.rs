use crate::internal_api::AuthType;
use crate::repository::Repository;

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
        team: &str,
    ) -> Result<PageIterator<Repository>, crate::error::Error> {
        self.get_paginated(&format!("{}/repositories/{}", self.url, team))
            .map(|v| v.get_all_pages(self))
    }
}

pub struct PageIterator<'a, T>
where
    for<'zut> T: serde::Deserialize<'zut>,
{
    pub(crate) next_page: Option<String>,
    pub(crate) current_page: Vec<T>,
    pub(crate) api: &'a crate::API,
}

impl<'a, T> Iterator for PageIterator<'a, T>
where
    for<'zut> T: serde::Deserialize<'zut>,
    T: std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.current_page.pop();
        match (result, &self.next_page) {
            (Some(result), _) => Some(result),
            (None, Some(ref url)) => {
                let next = self.api.get_paginated::<T>(url);
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

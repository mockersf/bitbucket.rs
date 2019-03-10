use serde::Deserialize;

use crate::api::API;

pub(crate) enum AuthType {
    Bearer { token: String },
    Basic { username: String, password: String },
}

impl API {
    pub(crate) fn authed_query(&self, url: &reqwest::Url) -> reqwest::RequestBuilder {
        let client = match self.client {
            Some(ref client) => client.clone(),
            None => reqwest::Client::new(),
        };
        let request_builder = client.get(url.as_ref());
        match self.auth_type {
            Some(AuthType::Bearer { ref token }) => request_builder.bearer_auth(token),
            Some(AuthType::Basic {
                ref username,
                ref password,
            }) => request_builder.basic_auth(username, Some(password)),
            None => request_builder,
        }
    }

    pub(crate) fn get_page<T>(&self, url: &reqwest::Url) -> Result<Page<T>, crate::error::Error>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let text = self
            .authed_query(url)
            .send()
            .map_err(|error| crate::error::Error::Http {
                url: String::from(url.as_ref()),
                error,
            })
            .and_then(|mut response| {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let error = response.text().map_err(|_| ()).and_then(|text| {
                        serde_json::de::from_str::<BitbucketErrorWrapper>(&text).map_err(|_| ())
                    });
                    Err(crate::error::Error::ErrorResponse {
                        url: String::from(url.as_ref()),
                        status_code: response.status(),
                        message: match error {
                            Ok(error) => error.error.message,
                            _ => String::from("unspecified error"),
                        },
                    })
                }
            })?
            .text()
            .map_err(|error| crate::error::Error::Http {
                url: String::from(url.as_ref()),
                error,
            })?;
        Ok(serde_json::de::from_str::<Page<T>>(&text)
            .map_err(|err| crate::error::Error::Deserialization(err))?)
    }

    pub(crate) fn get_paginated<T>(
        &self,
        url: reqwest::Url,
    ) -> Result<crate::Paginated<T>, crate::error::Error>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let page = self.get_page(&url)?;
        Ok(crate::Paginated {
            has_more: page.next.is_some(),
            current_page: page,
            client: self,
        })
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Page<T> {
    pagelen: u8,
    size: Option<u16>,
    page: u8,
    pub(crate) next: Option<String>,
    previous: Option<String>,
    pub(crate) values: Vec<T>,
}

impl<T> Page<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub(crate) fn get_all_pages(self, api: &crate::API) -> crate::PageIterator<T> {
        crate::PageIterator {
            next_page: self.next,
            current_page: self.values,
            api,
        }
    }
}

#[derive(Deserialize)]
struct BitbucketErrorWrapper {
    error: BitbucketError,
}

#[derive(Deserialize)]
struct BitbucketError {
    message: String,
}

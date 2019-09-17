use reqwest::{Url, Response};
use log::error;

pub mod unsplash {
    use reqwest::Url;
    use serde::Deserialize;

    const COLLECTIONS_BASE_URL: &str = "https://api.unsplash.com/collections";

    #[derive(Deserialize, Debug)]
    pub struct PhotoSizes {
        pub raw: Option<String>,
        pub full: Option<String>,
        pub regular: Option<String>,
        pub small: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct PhotoMeta {
        pub urls: PhotoSizes,
    }

    pub type Photos = Vec<PhotoMeta>;

    #[derive(Clone)]
    pub struct CollectionEndpoint {
        url: Url,
        client_id: Option<String>,
        current_page: usize,
    }

    impl CollectionEndpoint {

        pub fn new(collection_id: u128) -> Self {
            let url = Url::parse(&format!("{}/{}/photos/", COLLECTIONS_BASE_URL, collection_id))
                .unwrap();
            Self {
                url,
                client_id: None,
                current_page: 1,
            }
        }

        pub fn set_client_id(mut self, client_id: String) -> Self {
            self.client_id = Some(client_id);
            self
        }

        pub fn with_page(mut self, page: usize) -> Self {
            self.set_page(page);
            self
        }

        pub fn set_page(&mut self, page: usize) {
            self.current_page = page;
        }

        pub fn get_url(&self) -> String {
            let mut url = self.url.clone();
            url.query_pairs_mut()
                .append_pair("page", self.current_page.to_string().as_ref());
            if let Some(client_id) = &self.client_id {
                url.query_pairs_mut().append_pair("client_id", &client_id);
            }
            url.as_str().to_owned()
        }

        pub fn fetch_photos(&self, client: &reqwest::Client) -> reqwest::Result<Photos> {
            client.get(self.get_url().as_str()).send()?.json()
        }
    }

    pub struct PagesIterator {
        endpoint: CollectionEndpoint,
        current_page: usize,
    }

    impl IntoIterator for CollectionEndpoint {
        type Item = Photos;
        type IntoIter = PagesIterator;

        fn into_iter(self) -> Self::IntoIter {
            let current_page = self.current_page;
            PagesIterator {
                endpoint: self,
                current_page
            }
        }
    }

    impl Iterator for PagesIterator {
        type Item = Photos;

        fn next(&mut self) -> Option<Self::Item> {
            info!("Fetching new page: {}", self.endpoint.get_url());
            let client = reqwest::Client::new();
            let photos = self.endpoint.fetch_photos(&client);
            match photos {
                // TODO error handling should be improved
                Ok(photos) => {
                    self.endpoint.set_page(self.endpoint.current_page + 1);
                    if photos.len() == 0 { None } else { Some(photos) }
                },
                Err(e) => {
                    error!("Failed to fetch next page, reason: {}", e);
                    None
                }
            }
        }
    }

    #[derive(new)]
    pub struct PhotoSource {
        pages_iterator: PagesIterator,
    }

    impl IntoIterator for PhotoSource {
        type Item = Url;
        type IntoIter = impl Iterator<Item = Url>;

        fn into_iter(self) -> Self::IntoIter {
            self.pages_iterator
                .flat_map(|p| p.into_iter())
                .filter_map(|p| p.urls.raw)
                .map(|url| Url::parse(&url).unwrap())
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_collection_endpoint() {
            let endpoint = CollectionEndpoint::new(1234);
            let expected_url = format!("{}/{}/photos/", COLLECTIONS_BASE_URL, 1234);
            assert_eq!(endpoint.get_url(), expected_url);
            let endpoint = endpoint.set_client_id("myid".to_owned());
            let expected_url = format!("{}?client_id=myid", expected_url);
            assert_eq!(endpoint.get_url(), expected_url);
        }
    }
}

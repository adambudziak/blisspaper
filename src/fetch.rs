pub mod unsplash {
    use serde::Deserialize;

    const COLLECTIONS: &str = "https://api.unsplash.com/collections";

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

    pub struct CollectionEndpoint {
        url: String,
        client_id: Option<String>,
    }

    impl CollectionEndpoint {
        pub fn new(collection_id: u64) -> Self {
            Self {
                url: format!("{}/{}/photos/", COLLECTIONS, collection_id),
                client_id: None,
            }
        }

        pub fn with_client_id(self, client_id: String) -> Self {
            Self {
                url: self.url,
                client_id: Some(client_id),
            }
        }

        pub fn get_url(&self) -> String {
            match &self.client_id {
                Some(c_id) => format!("{}?client_id={}", self.url, c_id),
                None => self.url.clone(),
            }
        }

        pub fn fetch_photos(&self, client: &reqwest::Client) -> reqwest::Result<Photos> {
            client.get(self.get_url().as_str()).send()?.json()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_collection_endpoint() {
            let endpoint = CollectionEndpoint::new(1234);
            let expected_url = format!("{}/{}/photos/", COLLECTIONS, 1234);
            assert_eq!(endpoint.get_url(), expected_url);
            let endpoint = endpoint.with_client_id("myid".to_owned());
            let expected_url = format!("{}?client_id=myid", expected_url);
            assert_eq!(endpoint.get_url(), expected_url);
        }
    }
}

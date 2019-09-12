#[macro_use]
extern crate derive_new;

pub mod fetch;
pub mod store;
pub mod wallpaper;
pub mod bliss;
pub mod config;

use std::path::Path;

use serde::Deserialize;

const API_KEYS_FILE: &str = "api_keys.yml";

#[derive(Deserialize, Debug)]
pub struct ApiKeys {
    pub unsplash_client_id: String,
}

impl ApiKeys {
    pub fn load() -> Self {
        let path = Path::new(API_KEYS_FILE);
        assert!(path.exists(), "api_keys.yml file does not exist! Quitting");

        let content = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&content).expect("Invalid api_keys.yml")
    }
}

use serde::Deserialize;
use std::path::Path;

const CONFIG_FILE: &str = "bliss_config.yml";

#[derive(Deserialize)]
pub struct Config {
    pub changerate: u64,
    pub collections: Vec<u128>,
}

impl Config {
    pub fn load() -> Config {
        let path = Path::new(CONFIG_FILE);
        assert!(path.exists(), format!("{} file does not exist! Quitting.", CONFIG_FILE));

        let content = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&content).expect("Invalid config file.")
    }
}

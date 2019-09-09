pub mod fetch;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use crate::fetch::unsplash::Photos;
use serde::Deserialize;

const API_KEYS_FILE: &str = "api_keys.yml";

const WALLPAPERS_DEFAULT_STORAGE: &str = "/home/abudziak/.blisspaper/wallpapers";

#[derive(Deserialize, Debug)]
pub struct ApiKeys {
    pub unsplash_client_id: String,
}

pub fn set_wallpaper(wallpaper_path: &str) {
    let output = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.background", "picture-uri"])
        .arg(wallpaper_path)
        .output()
        .expect("Couldn't set the wallpaper");
    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
}

pub fn set_screensaver(screensaver_path: &str) {
    let output = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.screensaver", "picture-uri"])
        .arg(screensaver_path)
        .output()
        .expect("Couldn't set the screensaver");
    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
}

pub fn create_wallpaper_storage_if_not_exists() -> bool {
    let store_path = Path::new(WALLPAPERS_DEFAULT_STORAGE);
    if !store_path.exists() {
        std::fs::create_dir_all(store_path).expect("Couldn't create directory");
        true
    } else {
        false
    }
}

pub fn load_api_keys() -> ApiKeys {
    let path = Path::new(API_KEYS_FILE);
    assert!(path.exists(), "api_keys.yml file does not exist! Quitting");

    let content = std::fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content).expect("Invalid api_keys.yml")
}

pub fn default_wallpaper_path(filename: &str) -> String {
    Path::new(WALLPAPERS_DEFAULT_STORAGE)
        .join(filename)
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn save_wallpaper_from_response(
    wallpaper_path: &str,
    response: &mut reqwest::Response,
) -> reqwest::Result<()> {
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf)?;

    let mut f = File::create(&wallpaper_path).unwrap();
    f.write_all(buf.as_slice())
        .expect("Couldn't write the wallpaper to file");
    f.sync_data().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

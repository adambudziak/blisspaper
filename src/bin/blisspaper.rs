use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::path::Path;

use serde::Deserialize;

const UNSPLASH_ENDPOINT: &str = "https://api.unsplash.com/collections/1053828/photos/?client_id=";
const API_KEYS_FILE: &str = "api_keys.yml";

const WALLPAPERS_DEFAULT_STORAGE: &str = "/home/abudziak/.blisspaper/wallpapers";

#[derive(Deserialize, Debug)]
struct ApiKeys {
    unsplash_client_id: String,
}

#[derive(Deserialize, Debug)]
struct PhotoSizes {
    raw: Option<String>,
    full: Option<String>,
    regular: Option<String>,
    small: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PhotoMeta {
    urls: PhotoSizes,
}

type Photos = Vec<PhotoMeta>;

fn set_wallpaper(wallpaper_path: &str) {
    let output = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.background", "picture-uri"])
        .arg(wallpaper_path)
        .output()
        .expect("Couldn't set the wallpaper");
    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
}

fn set_screensaver(screensaver_path: &str) {
    let output = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.screensaver", "picture-uri"])
        .arg(screensaver_path)
        .output()
        .expect("Couldn't set the screensaver");
    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
}

fn create_wallpaper_storage_if_not_exists() -> bool {
    let store_path = Path::new(WALLPAPERS_DEFAULT_STORAGE);
    if !store_path.exists() {
        std::fs::create_dir_all(store_path).expect("Couldn't create directory");
        true
    } else {
        false
    }
}

fn load_api_keys() -> ApiKeys {
    let path = Path::new(API_KEYS_FILE);
    assert!(path.exists(), "api_keys.yml file does not exist! Quitting");

    let content = std::fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content).expect("Invalid api_keys.yml")
}


fn main() -> Result<(), reqwest::Error> {
    let api_keys = load_api_keys();
    if create_wallpaper_storage_if_not_exists() {
        println!("Wallpaper directory didn't exist. Created a new one..");
    } else {
        println!("Found existing wallpaper directory.");
    }
    let client = reqwest::Client::new();
    let mut res = client.get(&(UNSPLASH_ENDPOINT.to_owned() + &api_keys.unsplash_client_id)).send()?;
    let photos: Photos = res.json()?;

    if photos.is_empty() {
        println!("No photos downloaded");
        return Ok(());
    }

    let first_photo_url = photos.as_slice()[0].urls.raw.as_ref().unwrap();
    let mut photo = client.get(first_photo_url).send()?;

    let mut buf: Vec<u8> = vec![];
    photo.copy_to(&mut buf)?;

    let wallpaper_path = Path::new(WALLPAPERS_DEFAULT_STORAGE)
        .join("wallpaper.jpg")
        .to_str().unwrap().to_owned();

    let mut f = File::create(&wallpaper_path).unwrap();
    f.write_all(buf.as_slice()).expect("Couldn't write the wallpaper to file");
    f.sync_data().unwrap();

    set_wallpaper(&wallpaper_path);
    set_screensaver(&wallpaper_path);

    Ok(())
}

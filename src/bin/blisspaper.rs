use blisspaper::{
    create_wallpaper_storage_if_not_exists, default_wallpaper_path, load_api_keys,
    load_unsplash_photos, save_wallpaper_from_response, set_screensaver, set_wallpaper, Photos,
};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), reqwest::Error> {
    let api_keys = load_api_keys();
    if create_wallpaper_storage_if_not_exists() {
        println!("Wallpaper directory didn't exist. Created a new one..");
    } else {
        println!("Found existing wallpaper directory.");
    }
    let client = reqwest::Client::new();
    let photos = load_unsplash_photos(&client, &api_keys)?;

    if photos.is_empty() {
        println!("No photos downloaded");
        return Ok(());
    }

    let first_photo_url = photos.as_slice()[0].urls.raw.as_ref().unwrap();
    let mut photo = client.get(first_photo_url).send()?;

    let wallpaper_path = default_wallpaper_path("wallpaper.jpg");
    save_wallpaper_from_response(&wallpaper_path, &mut photo)?;

    set_wallpaper(&wallpaper_path);
    set_screensaver(&wallpaper_path);

    Ok(())
}

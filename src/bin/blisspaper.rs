use blisspaper::{
    create_wallpaper_storage_if_not_exists, default_wallpaper_path, load_api_keys,
    save_wallpaper_from_response, set_screensaver, set_wallpaper,
};
use flexi_logger::LogSpecification;
use log::{info, LevelFilter};
use blisspaper::fetch::unsplash;

fn main() -> reqwest::Result<()> {
    let log_spec = LogSpecification::default(LevelFilter::Info).build();
    flexi_logger::Logger::with(log_spec).start().unwrap();
    let api_keys = load_api_keys();
    if create_wallpaper_storage_if_not_exists() {
        info!("Wallpaper directory didn't exist. Created a new one..");
    } else {
        info!("Found existing wallpaper directory.");
    }
    let client = reqwest::Client::new();
    let photos = unsplash::CollectionEndpoint::new(1053828)
        .with_client_id(api_keys.unsplash_client_id.clone())
        .fetch_photos(&client)?;

    if photos.is_empty() {
        info!("No photos downloaded");
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

use blisspaper::fetch::unsplash;
use blisspaper::store::{Store, StoreError};
use blisspaper::{load_api_keys, wallpaper};
use blisspaper::bliss::Bliss;
use flexi_logger::LogSpecification;
use log::{info, LevelFilter};
use blisspaper::wallpaper::{Wallpaper, Screensaver};

fn main() -> reqwest::Result<()> {
    let log_spec = LogSpecification::default(LevelFilter::Info).build();
    flexi_logger::Logger::with(log_spec).start().unwrap();
    let api_keys = load_api_keys();
    let store = Store::default();
    if store.create_dir() {
        info!("Wallpaper directory didn't exist, created a new one.");
    } else {
        info!("Found an existing wallpaper directory.");
    }
    let client = reqwest::Client::new();
    let photos = unsplash::CollectionEndpoint::new(1053828)
        .with_client_id(api_keys.unsplash_client_id.clone())
        .fetch_photos(&client)?;

    if photos.is_empty() {
        info!("No photos downloaded");
        return Ok(());
    }

    let manager = wallpaper::gnome::Manager;
    let mut bliss = Bliss::new(manager, store, photos.into());
    bliss.run();

    Ok(())
}

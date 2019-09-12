use blisspaper::fetch::unsplash;
use blisspaper::store::{Store, StoreError};
use blisspaper::{wallpaper, ApiKeys};
use blisspaper::bliss::Bliss;
use flexi_logger::LogSpecification;
use log::{info, LevelFilter};
use blisspaper::wallpaper::{Wallpaper, Screensaver};
use blisspaper::config::Config;

fn main() -> reqwest::Result<()> {
    let log_spec = LogSpecification::default(LevelFilter::Info).build();
    flexi_logger::Logger::with(log_spec).start().unwrap();

    let config = Config::load();
    let api_keys = ApiKeys::load();
    let store = Store::default();

    if store.create_dir() {
        info!("Wallpaper directory didn't exist, created a new one.");
    } else {
        info!("Found an existing wallpaper directory.");
    }

    let client = reqwest::Client::new();
    let photos = unsplash::CollectionEndpoint::new(config.collections[0])
        .with_client_id(api_keys.unsplash_client_id.clone())
        .fetch_photos(&client)?;

    if photos.is_empty() {
        info!("No photos downloaded");
        return Ok(());
    }

    let manager = wallpaper::gnome::Manager;
    let mut bliss = Bliss::new(manager, store, photos.into())
        .set_changerate(config.changerate);
    bliss.run();

    Ok(())
}

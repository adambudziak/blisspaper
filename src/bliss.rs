use log::{error, info, warn};
use std::fs::{read_dir, DirEntry};

use crate::fetch::unsplash::{CollectionEndpoint, PhotoSource};
use crate::store::{Store, StoreError};
use crate::wallpaper::{Screensaver, Wallpaper, WallpaperAndScreensaver};
use reqwest::Url;

pub struct Bliss {
    manager: Box<dyn WallpaperAndScreensaver>,
    store: Store,
    endpoint: CollectionEndpoint,
    changerate: u64,
}

impl Bliss {
    pub fn new(
        manager: Box<dyn WallpaperAndScreensaver>,
        store: Store,
        endpoint: CollectionEndpoint,
    ) -> Self {
        Self {
            manager,
            store,
            endpoint,
            changerate: 5,
        }
    }

    pub fn fetch_new_wallpaper(
        &self,
        client: &reqwest::Client,
        photo_iter: &mut impl Iterator<Item = Url>,
    ) -> Option<()> {
        match photo_iter.next() {
            Some(photo_url) => {
                info!("Fetching next wallpaper: {}", photo_url);
                let mut response = client.get(photo_url.as_str()).send().unwrap();
                if let Err(e) = self.store.save_wallpaper(&mut response) {
                    warn!(
                        "Couldn't save the wallpaper from {} in the store. Reason: {:?}",
                        photo_url, e
                    )
                }
                Some(())
            }
            None => None,
        }
    }

    pub fn change_wallpaper(
        &self,
        wallpaper_iter: &mut impl Iterator<Item = std::io::Result<DirEntry>>,
    ) -> Option<()> {
        match wallpaper_iter.next() {
            Some(Err(e)) => {
                error!("Couldn't get the wallpaper from the store, reason: {}", e);
                Some(())
            }
            Some(Ok(entry)) => {
                let path = entry.path().to_str().unwrap().to_owned();

                self.manager.set_wallpaper(&path);
                self.manager.set_screensaver(&path);
                Some(())
            }
            None => None,
        }
    }

    fn init_photo_iter(&self) -> impl Iterator<Item = Url> + '_ {
        PhotoSource::new(self.endpoint.clone().into_iter())
            .into_iter()
            .filter(move |url| !self.store.contains(&url))
    }

    pub fn run(&mut self) {
        let sleep_duration = std::time::Duration::from_secs(self.changerate);
        let client = reqwest::Client::new();

        let mut photo_iter = self.init_photo_iter();
        let mut wallpaper_iter = self.store.iter_wallpapers().unwrap();
        loop {
            if self.fetch_new_wallpaper(&client, &mut photo_iter).is_none() {
                photo_iter = self.init_photo_iter();
            }
            if self.change_wallpaper(&mut wallpaper_iter).is_none() {
                wallpaper_iter = self.store.iter_wallpapers().unwrap();
            }
            std::thread::sleep(sleep_duration);
        }
    }

    pub fn set_changerate(mut self, changerate: u64) -> Self {
        self.changerate = changerate;
        self
    }
}

use log::{error, info, warn};
use std::fs::DirEntry;

use crate::fetch::unsplash::{CollectionEndpoint, PhotoSource};
use crate::store::Store;
use crate::wallpaper::WallpaperAndScreensaver;
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

    fn fetch_new_wallpaper(&self, client: &reqwest::Client, photo_url: Url) {
        info!("Fetching next wallpaper: {}", photo_url);
        let mut response = client.get(photo_url.as_str()).send().unwrap();
        if let Err(e) = self.store.save_wallpaper(&mut response) {
            warn!(
                "Couldn't save the wallpaper from {} in the store. Reason: {:?}",
                photo_url, e
            );
        }
    }

    fn change_wallpaper(&self, next_wallpaper: DirEntry) {
        let path = next_wallpaper.path().to_str().unwrap().to_owned();

        // those are unwraps no-op for now
        self.manager.set_wallpaper(&path).unwrap();
        self.manager.set_screensaver(&path).unwrap();
    }

    fn remove_oldest_wallpaper(&self) {
        match self.store.oldest_wallpaper() {
            Some(de) => {
                let path = de.path().to_str().unwrap().to_owned();
                info!("Removing oldest wallpaper: {}", path);
                if let Err(e) = std::fs::remove_file(&path) {
                    error!("Failed to remove wallpaper {}, reason: {:?}", path, e);
                }
            }
            None => panic!(
                "Attempted to remove the oldest wallpaper while the store is empty.
                 This is a bug"
            ),
        }
    }

    fn init_photo_iter(&self, client: &reqwest::Client) -> impl Iterator<Item = Url> + '_ {
        PhotoSource::new(self.endpoint.clone().into_pages_iterator(client.clone()))
            .into_iter()
            .filter(move |url| !self.store.contains(&url))
    }

    pub fn run(&mut self) {
        let sleep_duration = std::time::Duration::from_secs(self.changerate);
        let client = reqwest::Client::builder().use_sys_proxy().build().unwrap();

        let mut photo_iter = self.init_photo_iter(&client);
        let mut wallpaper_iter = self.store.sorted_wallpapers();

        let store_size = self.store.size() as i32;

        // at first iterate the wallpapers until we run close to the end of the store
        (0_i32..store_size - 10)
            .zip(&mut wallpaper_iter)
            .for_each(|(_, wallpaper)| {
                self.change_wallpaper(wallpaper);
                std::thread::sleep(sleep_duration);
            });

        // now we want to constantly fetch new wallpapers and remove the oldest one
        loop {
            let store_available = match self.store.capacity.checked_sub(self.store.size()) {
                Some(s) => s,
                None => 0,
            };
            info!("Store available: {}", store_available);
            if store_available == 0 {
                self.remove_oldest_wallpaper();
            }
            match photo_iter.next() {
                Some(photo_url) => self.fetch_new_wallpaper(&client, photo_url),
                None => {
                    photo_iter = self.init_photo_iter(&client);
                }
            }
            match wallpaper_iter.next() {
                Some(wallpaper) => self.change_wallpaper(wallpaper),
                None => {
                    wallpaper_iter = self.store.sorted_wallpapers();
                }
            }
            std::thread::sleep(sleep_duration);
        }
    }

    pub fn set_changerate(mut self, changerate: u64) -> Self {
        self.changerate = changerate;
        self
    }
}

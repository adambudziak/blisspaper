use std::fs::read_dir;
use log::{info, warn, error};

use crate::wallpaper::{Screensaver, Wallpaper};
use crate::store::{Store, StoreError};
use crate::fetch::unsplash::{PhotoSource, CollectionEndpoint};

pub struct Bliss<M: Wallpaper + Screensaver> {
    manager: M,
    store: Store,
    endpoint: CollectionEndpoint,
    changerate: u64,
}

impl<M: Wallpaper + Screensaver> Bliss<M> {
    pub fn new(manager: M, store: Store, endpoint: CollectionEndpoint) -> Self {
        Self { manager, store, endpoint, changerate: 5 }
    }

    pub fn run(&mut self) {
        let sleep_duration = std::time::Duration::from_secs(self.changerate);
        let client = reqwest::Client::new();

        loop {
            let photo_source = PhotoSource::new(self.endpoint.clone().into_iter());
            for photo_url in photo_source.into_iter() {
                let mut response = client.get(photo_url.as_str()).send().unwrap();
                // TODO add error handling as soon as store is capable of returning
                //      something useful
//                match self.store.save_wallpaper(&mut response) {
//                    Ok(p) | Err(StoreError::WallpaperAlreadyExists(p)) => (),
//                    Err(e) => {
//
//                    }
//                }
                if let Err(e) = self.store.save_wallpaper(&mut response) {
                    warn!("Couldn't save the wallpaper from {} in the store. Reason: {:?}", photo_url, e)
                }


                let wallpapers = self.store.iter_wallpapers().unwrap();
                for wallpaper in wallpapers {

                    if let Err(e) = wallpaper {
                        error!("Couldn't get the wallpaper from the store, reason: {}", e);
                        continue
                    }

                    let path = wallpaper.unwrap().path().to_str().unwrap().to_owned();

                    self.manager.set_wallpaper(&path);
                    self.manager.set_screensaver(&path);

                    std::thread::sleep(sleep_duration);
                }
            }
        }
    }

    pub fn set_changerate(mut self, changerate: u64) -> Self {
        self.changerate = changerate;
        self
    }
}

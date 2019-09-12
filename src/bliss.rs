use crate::wallpaper::{Screensaver, Wallpaper};
use crate::store::Store;
use crate::fetch::PhotoSource;

pub struct Bliss<M: Wallpaper + Screensaver> {
    manager: M,
    store: Store,
    source: PhotoSource,
    changerate: u64,
}

impl<M: Wallpaper + Screensaver> Bliss<M> {
    pub fn new(manager: M, store: Store, source: PhotoSource) -> Self {
        Self { manager, store, source, changerate: 5 }
    }

    pub fn run(&mut self) {
        let sleep_duration = std::time::Duration::from_secs(self.changerate);
        let client = reqwest::Client::new();
        let photos = self.source.urls.iter().cycle();

        for photo in photos {

            let path = if !self.store.contains(photo) {
                let mut response = self.source.download(&client, photo).unwrap();
                let save_result = self.store.save_wallpaper(&mut response);
                match save_result {
                    Ok(path) => path,
                    Err(e) => panic!("{:?}", e)
                }
            } else {
                self.store.get_filepath(photo).to_str().unwrap().to_owned()
            };

            self.manager.set_wallpaper(&path);
            self.manager.set_screensaver(&path);
            std::thread::sleep(sleep_duration);
        }
    }

    pub fn set_changerate(mut self, changerate: u64) -> Self {
        self.changerate = changerate;
        self
    }
}

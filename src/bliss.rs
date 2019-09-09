use crate::wallpaper::{Screensaver, Wallpaper};
use crate::store::Store;
use crate::fetch::PhotoSource;

#[derive(new)]
pub struct Bliss<M: Wallpaper + Screensaver> {
    manager: M,
    store: Store,
    source: PhotoSource,
}

impl<M: Wallpaper + Screensaver> Bliss<M> {
    pub fn run(&mut self) {
        let sleep_duration = std::time::Duration::from_secs(5);
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
            std::thread::sleep(sleep_duration);
        }
    }
}

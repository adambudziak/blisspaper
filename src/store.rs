use std::fs::{File, ReadDir, read_dir};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

const WALLPAPERS_DEFAULT_STORAGE: &str = ".blisspaper/wallpapers";

#[derive(Debug)]
pub enum StoreError {
    WallpaperAlreadyExists(String),
}

#[derive(Debug)]
pub struct Store {
    store_path: PathBuf,
}

impl Default for Store {
    fn default() -> Self {
        let path = Path::new(env!("HOME")).join(WALLPAPERS_DEFAULT_STORAGE);
        Self { store_path: path }
    }
}

impl Store {
    pub fn create_dir(&self) -> bool {
        if !self.store_path.exists() {
            std::fs::create_dir_all(&self.store_path).expect("Couldn't create directory");
            true
        } else {
            false
        }
    }

    pub fn contains(&self, url: &reqwest::Url) -> bool {
        self.get_filepath(url).exists()
    }

    pub fn get_filepath(&self, url: &reqwest::Url) -> PathBuf {
        // TODO detect the extension
        let filename = base64::encode(url.as_str()) + ".jpg";
        self.store_path.join(filename)
    }

    pub fn save_wallpaper(&self, response: &mut reqwest::Response) -> Result<String, StoreError> {
        let filepath = self.get_filepath(response.url());
        let filepath_str = filepath.to_str().unwrap().to_owned();
        if filepath.exists() {
            return Err(StoreError::WallpaperAlreadyExists(filepath_str))
        }
        let mut writer = BufWriter::new(File::create(&filepath).unwrap());
        response.copy_to(&mut writer).unwrap();
        Ok(filepath_str)
    }

    pub fn iter_wallpapers(&self) -> std::io::Result<ReadDir> {
        read_dir(&self.store_path)
    }
}

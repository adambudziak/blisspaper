use std::fs::{read_dir, File, ReadDir};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

const WALLPAPERS_DEFAULT_STORAGE: &str = ".blisspaper/wallpapers";

#[derive(Debug)]
pub enum StoreError {
    WallpaperAlreadyExists(String),
}

#[derive(Debug)]
pub struct Store {
    pub path: PathBuf,
    pub capacity: usize,
}

impl Default for Store {
    fn default() -> Self {
        let path = Path::new(env!("HOME")).join(WALLPAPERS_DEFAULT_STORAGE);
        Self {
            path: path,
            capacity: 10,
        }
    }
}

impl Store {
    pub fn create_dir(&self) -> bool {
        if !self.path.exists() {
            std::fs::create_dir_all(&self.path).expect("Couldn't create directory");
            true
        } else {
            false
        }
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    pub fn contains(&self, url: &reqwest::Url) -> bool {
        self.get_filepath(url).exists()
    }

    pub fn get_filepath(&self, url: &reqwest::Url) -> PathBuf {
        // TODO detect the extension
        let filename = base64::encode(url.as_str()).replace("/", "_") + ".jpg";
        self.path.join(filename)
    }

    /// Get the number of currently stored wallpapers.
    pub fn size(&self) -> usize {
        std::fs::read_dir(&self.path).unwrap().count()
    }

    pub fn save_wallpaper(&self, response: &mut reqwest::Response) -> Result<String, StoreError> {
        let filepath = self.get_filepath(response.url());
        let filepath_str = filepath.to_str().unwrap().to_owned();

        info!(
            "Saving image from {} to {}",
            response.url().as_str(),
            filepath_str
        );
        if filepath.exists() {
            return Err(StoreError::WallpaperAlreadyExists(filepath_str));
        }
        let mut writer = BufWriter::new(File::create(&filepath).unwrap());
        response.copy_to(&mut writer).unwrap();
        Ok(filepath_str)
    }

    pub fn remove_wallpaper(&self, filepath: &str) {
        std::fs::remove_file(filepath).unwrap();
    }

    pub fn iter_wallpapers(&self) -> std::io::Result<ReadDir> {
        read_dir(&self.path)
    }
}

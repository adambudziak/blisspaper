pub trait Wallpaper {
    fn set_wallpaper(&self, path: &str) -> Result<(), ()>;
}

pub trait Screensaver {
    fn set_screensaver(&self, path: &str) -> Result<(), ()>;
}

pub mod gnome {
    use crate::wallpaper::{Wallpaper, Screensaver};
    use std::process::Command;

    pub struct Manager;

    impl Wallpaper for Manager {
        fn set_wallpaper(&self, path: &str) -> Result<(), ()> {
            let _output = Command::new("gsettings")
                .args(&["set", "org.gnome.desktop.background", "picture-uri"])
                .arg(path)
                .output()
                .expect("Couldn't set the wallpaper");
            Ok(())
        }
    }

    impl Screensaver for Manager {
        fn set_screensaver(&self, path: &str) -> Result<(), ()> {
            let _output = Command::new("gsettings")
                .args(&["set", "org.gnome.desktop.screensaver", "picture-uri"])
                .arg(path)
                .output()
                .expect("Couldn't set the screensaver");
            Ok(())
        }
    }
}

pub mod i3 {
    use crate::wallpaper::{ Wallpaper, Screensaver};
    use std::process::Command;

    pub struct Manager;

    impl Wallpaper for Manager {
        fn set_wallpaper(&self, path: &str) -> Result<(), ()> {
            let _output = Command::new("feh")
                .arg("--bg-fill")
                .arg(path)
                .output()
                .expect("Couldn't set the wallpaper (is feh installed?).");
            Ok(())
        }
    }

    impl Screensaver for Manager {
        fn set_screensaver(&self, _path: &str) -> Result<(), ()> {
            Ok(())
        }
    }
}

pub mod dummy {
    use crate::wallpaper::{ Wallpaper, Screensaver};

    pub struct Manager;

    impl Wallpaper for Manager {
        fn set_wallpaper(&self, _path: &str) -> Result<(), ()> {
            Ok(())
        }
    }

    impl Screensaver for Manager {
        fn set_screensaver(&self, _path: &str) -> Result<(), ()> {
            Ok(())
        }
    }
}

/// Artificial trait to make boxing work.
///
/// For details, see https://github.com/rust-lang/rust/issues/32220.
pub trait WallpaperAndScreensaver: Wallpaper + Screensaver {}
impl<T> WallpaperAndScreensaver for T where T: Wallpaper + Screensaver {}

pub fn detect_session() -> Box<dyn WallpaperAndScreensaver> {
    let _session = env!("DESKTOP_SESSION").to_string().to_lowercase();

    match env!("DESKTOP_SESSION") {
        "i3" => {
            info!("Detected i3 session");
            Box::new(i3::Manager)
        },
        "gnome" => {
            info!("Detected gnome session");
            Box::new(gnome::Manager)
        },
        _ => {
            error!("Dummy wallpaper manager is being used. Session was not identified properly.");
            Box::new(dummy::Manager)
        }
    }
}

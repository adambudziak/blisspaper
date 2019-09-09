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
            let output = Command::new("gsettings")
                .args(&["set", "org.gnome.desktop.background", "picture-uri"])
                .arg(path)
                .output()
                .expect("Couldn't set the wallpaper");
            eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
            Ok(())
        }
    }

    impl Screensaver for Manager {
        fn set_screensaver(&self, path: &str) -> Result<(), ()> {
            let output = Command::new("gsettings")
                .args(&["set", "org.gnome.desktop.screensaver", "picture-uri"])
                .arg(path)
                .output()
                .expect("Couldn't set the screensaver");
            eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
            Ok(())
        }
    }
}

extern crate dirs;

use std::path::PathBuf;

pub trait Tilde {
    fn tilde_format(&self) -> String;
}

impl Tilde for PathBuf {
    fn tilde_format(&self) -> String {
        let path = self.display().to_string();
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir.display().to_string(),
            None => String::new(),
        };
        if path.starts_with(home_dir.as_str()) {
            path.replacen(home_dir.as_str(), "~", 1)
        } else {
            path
        }
    }
}

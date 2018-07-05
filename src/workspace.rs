extern crate serde;
extern crate serde_yaml;

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

impl Workspace {
    pub fn write(&self) -> &Self {
        const ERR_MESSAGE: &str = "ERROR: Could not write workspace data";

        let path = self.data_path();
        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path)
            .expect(ERR_MESSAGE);

        let serialized = serde_yaml::to_string(self).unwrap();
        file.write_fmt(format_args!("{}", serialized))
            .expect(ERR_MESSAGE);

        self
    }

    pub fn delete(&self) -> &Self {
        let path = self.data_path();
        fs::remove_file(path).expect("ERROR: Could not delete workspace data");
        self
    }

    pub fn exists(&self) -> bool {
        self.data_path().exists()
    }

    pub fn cd(&self) {
        run!("cd {}", self.path.display());
    }

    fn data_path(&self) -> PathBuf {
        let mut path = data_path();
        path.push(&self.name);
        path.set_extension("yaml");

        path
    }
}

pub fn get(name: &str) -> Option<Workspace> {
    paths()
        .into_iter()
        .filter(|path| {
            let file_stem = path.file_stem();
            if file_stem.is_none() {
                return false;
            }
            file_stem.unwrap().to_string_lossy() == name
        })
        .map(read)
        .map(parse)
        .nth(0)
}

pub fn all() -> Vec<Workspace> {
    files().into_iter().map(parse).collect()
}

pub fn parse(mut file: fs::File) -> Workspace {
    const ERR_MESSAGE: &str = "ERROR: could not read workspace data";

    let mut content = String::new();
    file.read_to_string(&mut content).expect(ERR_MESSAGE);

    serde_yaml::from_str(&content).expect(ERR_MESSAGE)
}

pub fn files() -> Vec<fs::File> {
    paths().into_iter().map(read).collect()
}

pub fn read(path: PathBuf) -> fs::File {
    fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("ERROR: could not get workspace data")
}

pub fn paths() -> Vec<PathBuf> {
    let entries = fs::read_dir(data_path()).expect("ERROR: could not find workspace data");
    let mut paths: Vec<PathBuf> = Vec::new();

    for entry in entries {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();

        if entry.file_type().is_err() {
            continue;
        }
        let file_type = entry.file_type().unwrap();
        if !file_type.is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().is_none() {
            continue;
        }
        let extension = path.extension().unwrap();
        if extension.to_string_lossy() != "yaml" {
            continue;
        }

        paths.push(entry.path());
    }

    paths
}

pub fn data_path() -> PathBuf {
    let mut path = env::home_dir().expect("ERROR: Could not find home directory");
    path.push(".workspace");

    if !path.exists() {
        fs::create_dir(&path)
            .unwrap_or_else(|_| panic!("ERROR: Could not create directory {}", path.display()))
    }

    path
}

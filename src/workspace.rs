extern crate serde;
extern crate serde_yaml;

use super::exit::Exit;
use super::VERBOSE;
use colored::*;
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
        const ERR_MESSAGE: &str = "Could not write workspace data";

        let path = self.data_path();
        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path)
            .unwrap_or_exit(ERR_MESSAGE);

        let serialized = serde_yaml::to_string(self).unwrap();
        file.write_fmt(format_args!("{}", serialized))
            .unwrap_or_exit(ERR_MESSAGE);

        self
    }

    pub fn delete(&self) -> &Self {
        let path = self.data_path();
        fs::remove_file(path).unwrap_or_exit("Could not delete workspace data");
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
    const ERR_MESSAGE: &str = "Could not read workspace data";

    let mut content = String::new();
    file.read_to_string(&mut content)
        .unwrap_or_exit(ERR_MESSAGE);

    serde_yaml::from_str(&content).unwrap_or_exit(ERR_MESSAGE)
}

pub fn files() -> Vec<fs::File> {
    paths().into_iter().map(read).collect()
}

pub fn read(path: PathBuf) -> fs::File {
    fs::OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_exit("Could not get workspace data")
}

pub fn paths() -> Vec<PathBuf> {
    let entries = fs::read_dir(data_path()).unwrap_or_exit("Could not find workspace data");
    let mut paths: Vec<PathBuf> = Vec::new();

    for entry in entries {
        skip_err!(entry);
        let entry = entry.unwrap();
        let path = entry.path();

        skip_err!(entry.file_type());
        let file_type = entry.file_type().unwrap();
        skip!(
            !file_type.is_file(),
            format!("Skipping {} because it's not a file", path.display())
        );

        skip_none!(
            path.extension(),
            format!(
                "Skipping {} because it has no file extension",
                path.display()
            )
        );
        let extension = path.extension().unwrap();
        skip!(
            extension.to_string_lossy() != "yaml",
            format!("Skipping {} because it's not a YAML file", path.display())
        );

        paths.push(entry.path());
    }

    paths
}

pub fn data_path() -> PathBuf {
    let mut path = env::home_dir().unwrap_or_exit("Could not find home directory");
    path.push(".workspace");

    if !path.exists() {
        fs::create_dir(&path)
            .unwrap_or_exit(&format!("Could not create directory {}", path.display()));
    }

    path
}

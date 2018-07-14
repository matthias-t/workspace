extern crate serde;
extern crate toml;

use super::exit::Exit;
use super::VERBOSE;
use colored::*;
use failure::Error;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

impl Workspace {
    pub fn write(&self) -> &Self {
        const ERR_MESSAGE: &str = "Could not write workspace data";

        let path = file_path(&self.name);
        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path)
            .unwrap_or_exit(ERR_MESSAGE);

        let serialized = toml::to_string(self).unwrap();
        file.write_fmt(format_args!("{}", serialized))
            .unwrap_or_exit(ERR_MESSAGE);

        self
    }

    pub fn delete(&self) -> &Self {
        let path = file_path(&self.name);
        fs::remove_file(path).unwrap_or_exit("Could not delete workspace data");
        self
    }

    pub fn exists(&self) -> bool {
        file_path(&self.name).exists()
    }

    pub fn cd(&self) {
        run!("cd {}", self.path.display());
    }
}

pub fn get(name: &str) -> Result<Workspace, Error> {
    parse(file_path(name))
}

pub fn all() -> Vec<(String, Result<Workspace, Error>)> {
    paths()
        .into_iter()
        .map(|path| {
            // Safe to unwrap here, because paths() cannot contain a file without a stem
            let name = path
                .file_stem()
                .unwrap()
                .to_str()
                .map(|slice| slice.to_string())
                .unwrap_or(format!(
                    "{} workspace name is invalid UTF-8",
                    "warning:".bold().yellow()
                ));
            (name, path)
        })
        .map(|(name, path)| (name, parse(path)))
        .collect()
}

fn parse(path: PathBuf) -> Result<Workspace, Error> {
    let content: String = read(path)?;
    let ws: Workspace = toml::from_str(&content)?;
    Ok(ws)
}

fn read(path: PathBuf) -> io::Result<String> {
    let mut content: String = String::new();

    fs::OpenOptions::new()
        .read(true)
        .open(path)?
        .read_to_string(&mut content)?;

    Ok(content)
}

fn paths() -> Vec<PathBuf> {
    let entries = fs::read_dir(folder_path()).unwrap_or_exit("Could not find workspace data");
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
            extension.to_string_lossy() != "toml",
            format!("Skipping {} because it's not a TOML file", path.display())
        );

        paths.push(entry.path());
    }

    paths
}

fn file_path(name: &str) -> PathBuf {
    folder_path().with_file_name(name).with_extension("toml")
}

fn folder_path() -> PathBuf {
    let mut path = env::home_dir().unwrap_or_exit("Could not find home directory");
    path.push(".workspace");

    if !path.exists() {
        fs::create_dir(&path)
            .unwrap_or_exit(&format!("Could not create directory {}", path.display()));
    }

    path
}

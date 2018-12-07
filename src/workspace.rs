use crate::exit::Exit;
use crate::tilde::Tilde;
use crate::VERBOSE;

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{self, Stdio};

use colored::Colorize;
use failure::Fail;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub path: PathBuf,
    #[serde(default)]
    pub tabs: Vec<String>,
    #[serde(default)]
    pub commands: Commands,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Commands {
    #[serde(default)]
    pub local: Vec<String>,
    #[serde(default)]
    pub external: Vec<String>,
    #[serde(default)]
    pub background: Vec<String>,
}

impl Workspace {
    pub fn open(&self, dir_only: bool) {
        run!("cd {}", self.path.display());
        if dir_only {
            return;
        }

        for command in &self.commands.local {
            run!("{}", command);
        }

        if !self.commands.external.is_empty() {
            if let Ok(terminal) = env::var("TERMINAL") {
                for command in &self.commands.external {
                    let result = process::Command::new(&terminal)
                        .arg(command)
                        .current_dir(&self.path)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn();

                    if result.is_err() {
                        error!("Could not run command: {}", command);
                        log!("{}", result.unwrap_err());
                    }
                }
            } else {
                error!("Please set $TERMINAL to run external commands");
            }
        }

        if !&self.commands.background.is_empty() {
            if let Ok(shell) = env::var("SHELL") {
                for command in &self.commands.background {
                    let result = process::Command::new(&shell)
                        .arg("-c")
                        .arg(command)
                        .current_dir(&self.path)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn();

                    if result.is_err() {
                        error!("Could not run command: {}", command);
                        log!("{}", result.unwrap_err());
                    }
                }
            } else {
                error!("Please set $SHELL to run commands in the background.");
            }
        }

        if !self.tabs.is_empty() {
            if let Ok(browser) = env::var("BROWSER") {
                for tab in &self.tabs {
                    let result = process::Command::new(&browser)
                        .arg(tab)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn();

                    if result.is_err() {
                        error!("Could not open tab: {}", tab);
                        log!("{}", result.unwrap_err())
                    }
                }
            } else {
                error!("Please set $BROWSER to open browser tabs")
            }
        }
    }

    pub fn write(&self, name: &str) {
        const ERR_MESSAGE: &str = "Could not write workspace data";

        let path = Self::file_path(name);
        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path)
            .unwrap_or_exit(ERR_MESSAGE);

        let serialized = toml::to_string(self).unwrap();
        file.write_fmt(format_args!("{}", serialized))
            .unwrap_or_exit(ERR_MESSAGE);
    }

    pub fn edit(name: &str) {
        let path = Self::file_path(name);
        let editor = env::var("EDITOR").unwrap_or_else(|_| {
            env::var("VISUAL").unwrap_or_exit("Please set $EDITOR or $VISUAL to edit workspaces")
        });
        run!("{} {}", editor, path.display());
    }

    pub fn delete(name: &str) {
        let path = Self::file_path(name);
        fs::remove_file(path).unwrap_or_exit("Could not delete workspace data");
    }

    pub fn exists(name: &str) -> bool {
        Self::file_path(name).exists()
    }

    pub fn get(name: &str) -> Option<Result<Workspace, Error>> {
        let path = Self::file_path(name);
        if !path.exists() {
            None
        } else {
            Some(Self::parse(&path))
        }
    }

    pub fn all() -> Vec<(Option<String>, Result<Workspace, Error>)> {
        Self::paths()
            .into_iter()
            .map(|path| {
                // Safe to unwrap here, because paths() cannot contain a file without a stem
                let name = path.file_stem().unwrap().to_str().map(str::to_owned);
                (name, path)
            }).map(|(name, path)| (name, Self::parse(&path)))
            .collect()
    }

    fn parse(path: &PathBuf) -> Result<Workspace, Error> {
        let content: String = Self::read(&path)?;
        let ws: Workspace = toml::from_str(&content)?;
        Ok(ws)
    }

    fn read(path: &PathBuf) -> io::Result<String> {
        let mut content: String = String::new();

        fs::OpenOptions::new()
            .read(true)
            .open(&path)?
            .read_to_string(&mut content)?;

        Ok(content)
    }

    fn paths() -> Vec<PathBuf> {
        let entries =
            fs::read_dir(Self::folder_path()).unwrap_or_exit("Could not find workspace data");
        let mut paths: Vec<PathBuf> = Vec::new();

        for entry in entries {
            skip_err!(entry);
            let entry = entry.unwrap();
            let path = entry.path();

            skip_err!(entry.file_type());
            let file_type = entry.file_type().unwrap();
            skip!(
                !file_type.is_file(),
                format!("Skipping {} because it's not a file", path.tilde_format())
            );

            skip_none!(
                path.extension(),
                format!(
                    "Skipping {} because it has no file extension",
                    path.tilde_format()
                )
            );
            let extension = path.extension().unwrap();
            skip!(
                extension.to_string_lossy() != "toml",
                format!(
                    "Skipping {} because it's not a TOML file",
                    path.tilde_format()
                )
            );

            paths.push(entry.path());
        }

        paths
    }

    pub fn file_path(name: &str) -> PathBuf {
        let mut path = Self::folder_path();
        path.push(name);
        path.set_extension("toml");
        path
    }

    fn folder_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_exit("Could not find configuration directory");
        path.push("workspace");

        if !path.exists() {
            fs::create_dir(&path).unwrap_or_exit(&format!(
                "Could not create directory {}",
                path.tilde_format()
            ));
        }

        path
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Could not read workspace data")]
    Read(#[cause] io::Error),
    #[fail(display = "Could not parse workspace data")]
    Parse(#[cause] toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::Read(cause)
    }
}

impl From<toml::de::Error> for Error {
    fn from(cause: toml::de::Error) -> Error {
        Error::Parse(cause)
    }
}

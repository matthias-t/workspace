#[macro_use]
pub mod macros;
mod app;
pub mod exit;
mod shell;
mod workspace;

extern crate clap;
extern crate colored;
#[macro_use]
extern crate failure;
extern crate term_grid;
#[macro_use]
extern crate serde_derive;

use clap::ArgMatches;
use colored::*;
use exit::*;
use failure::Fail;
use std::env;
use std::fs;
use std::io::Write;
use std::path;
use std::process;
use workspace::Workspace;

pub static mut VERBOSE: bool = false;

fn main() {
    let matches = app::cli().get_matches();

    unsafe {
        VERBOSE = matches.is_present("verbose");
    }

    if let Some(matches) = matches.subcommand_matches("open") {
        open(matches);
    } else if let Some(matches) = matches.subcommand_matches("add") {
        add(matches);
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        delete(matches);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        list();
    } else if let Some(matches) = matches.subcommand_matches("shell") {
        shell(matches);
    }
}

fn open(matches: &ArgMatches) {
    let name: &str = matches.value_of("NAME").unwrap();
    let result = Workspace::get(name)
        .unwrap_or_exit(&format!("A workspace called '{}' does not exist", name));
    let ws = result.unwrap_or_else(|error| {
        let path = Workspace::file_path(name);
        error!("{} from {}", error, path.display());
        if let Some(cause) = error.cause() {
            indent_error!("{}", cause);
        }
        if let Some(backtrace) = error.backtrace() {
            log!("{}", backtrace);
        }
        process::exit(1)
    });
    if !ws.path.exists() {
        error!("The location of this workspace does not exist anymore");
        indent_error!("the path '{}' was moved or deleted", ws.path.display());
        process::exit(1);
    }
    ws.open();
}

fn add(matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap().to_string();
    if Workspace::exists(&name) {
        error!("A workspace called '{}' already exists", name);
        process::exit(1);
    }
    let path = env::current_dir().unwrap_or_exit("Could not read current directory");

    // Check for other workspaces with the same path
    let sames: Vec<_> = Workspace::all()
        .into_iter()
        .filter_map(|(name, result)| {
            if let (Some(name), Ok(workspace)) = (name, result) {
                if workspace.path == path {
                    return Some(name);
                }
            }
            None
        })
        .collect();

    if !sames.is_empty() {
        warn!(
            "Found {} pointing to this directory: {}",
            if sames.len() == 1 {
                "another workspace"
            } else {
                "other workspaces"
            },
            sames.join(", ")
        );
        confirm!("Create a new workspace here anyway");
    }

    let ws = Workspace { path };
    ws.write(&name);
    println!("Created workspace '{}' in {}", name, ws.path.display());
}

fn delete(matches: &ArgMatches) {
    let name: &str = matches.value_of("NAME").unwrap();
    if !Workspace::file_path(name).exists() {
        error!("A workspace called '{}' does not exist", name);
        process::exit(1);
    }

    if !matches.is_present("yes") {
        confirm!("Delete the workspace '{}'", name);
    }

    Workspace::delete(name);
    println!("Deleted workspace '{}'", name);
}

fn list() {
    let all = Workspace::all();
    if all.is_empty() {
        println!("No existing workspaces.\nRun `workspace add <NAME>` to create one.");
        return;
    }

    use term_grid::{Direction, Filling, Grid, GridOptions};
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::LeftToRight,
    });

    for (name, result) in all {
        let path: String;
        let mut moved = String::new();
        match result {
            Ok(ws) => {
                path = ws.path.display().to_string().bright_black().to_string();
                if !ws.path.exists() {
                    moved = format!("{} path has moved", "warning:".bold().yellow());
                }
            }
            Err(error) => {
                path = format!("{} {}", "warning:".bold().yellow(), error);
            }
        }
        let name = name.unwrap_or_else(|| format!("{} invalid UTF-8", "warning:".bold().yellow()));

        grid.add(name.into());
        grid.add(path.into());
        grid.add(moved.into());
    }
    print!("{}", grid.fit_into_columns(3));
}

fn shell(matches: &ArgMatches) {
    if matches.subcommand_matches("bash").is_some() {
        println!("{}", shell::BASH);
    } else if matches.subcommand_matches("fish").is_some() {
        println!("{}", shell::FISH);
    } else if matches.subcommand_matches("powershell").is_some() {
        println!("{}", shell::POWERSHELL)
    } else if let Some(matches) = matches.subcommand_matches("cmd") {
        let mut path: path::PathBuf = path_to_binary_or_arg(&matches);
        let mut file: fs::File = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .truncate(true)
            .open(&path)
            .unwrap_or_exit(&format!(
                "Could not create batch file at {}",
                path.display()
            ));

        file.write_fmt(format_args!("{}", shell::CMD))
            .unwrap_or_exit("Could not write to batch file");

        println!("Wrote {}", path.display());
    }
}

fn path_to_binary_or_arg(matches: &ArgMatches) -> path::PathBuf {
    if let Some(path) = matches.value_of("PATH") {
        return path::Path::new(path)
            .with_file_name("ws")
            .with_extension("bat")
            .to_path_buf();
    } else {
        let mut path = env::current_exe().unwrap_or_exit("Could not determine path to binary");
        path.set_file_name("ws");
        path.set_extension("bat");
        return path;
    }
}

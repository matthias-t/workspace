#[macro_use]
pub mod macros;
mod app;
pub mod exit;
mod shell;
mod workspace;

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate colored;

use clap::ArgMatches;
use colored::*;
use exit::*;
use std::env;
use std::fs;
use std::io::{self, Write};
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
    let ws: Workspace = workspace::get(name)
        .unwrap_or_exit(&format!("A workspace called '{}' does not exist", name));
    ws.cd();
}

fn add(matches: &ArgMatches) {
    let ws = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().unwrap_or_exit("Could not read current directory"),
    };
    if ws.exists() {
        error!("A workspace called '{}' already exists", ws.name);
        process::exit(1);
    }
    ws.write();
    println!("Created workspace '{}' in {}", ws.name, ws.path.display());
}

fn delete(matches: &ArgMatches) {
    let ws = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().unwrap_or_exit("Could not read current directory"),
    };

    if !ws.exists() {
        error!("A workspace called '{}' does not exist", ws.name);
        process::exit(1);
    }

    if !matches.is_present("yes") {
        confirm!("delete the workspace '{}'", ws.name);
    }

    ws.delete();
    println!("Deleted workspace '{}' in {}", ws.name, ws.path.display());
}

fn list() {
    let all = workspace::all();

    if all.is_empty() {
        println!("No existing workspaces.\nRun `workspace add <NAME>` to create one.");
        return;
    }

    let longest_name_length = (*all).iter().map(|ws| ws.name.len()).fold(0, std::cmp::max);
    for ws in all {
        println!(
            "{0:<1$}  {2}",
            ws.name,
            longest_name_length,
            ws.path.display().to_string().bright_black()
        );
    }
}

fn shell(matches: &ArgMatches) {
    if matches.subcommand_matches("bash").is_some() {
        println!("{}", shell::BASH);
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

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
extern crate prettytable;
#[macro_use]
extern crate serde_derive;

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
    if !ws.path.exists() {
        error!("The location of this workspace does not exist anymore");
        println!("The path '{}' was moved or deleted", ws.path.display());
        process::exit(1);
    }
    ws.open();
}

fn add(matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap().to_string();
    if workspace::exists(&name) {
        error!("A workspace called '{}' already exists", name);
        process::exit(1);
    }
    let ws = Workspace {
        path: env::current_dir().unwrap_or_exit("Could not read current directory"),
    };
    workspace::write(&ws, &name);
    println!("Created workspace '{}' in {}", name, ws.path.display());
}

fn delete(matches: &ArgMatches) {
    let name: &str = matches.value_of("NAME").unwrap();

    if !matches.is_present("yes") {
        confirm!("delete the workspace '{}'", name);
    }

    workspace::delete(name);
    println!("Deleted workspace '{}'", name);
}

fn list() {
    let all = workspace::all();
    if all.is_empty() {
        println!("No existing workspaces.\nRun `workspace add <NAME>` to create one.");
        return;
    }

    use prettytable::{cell::Cell, format, row::Row, Table};
    let rows: Vec<Row> = all
        .iter()
        .map(|(name, result)| {
            let path: String;
            let mut moved: String = String::default();
            match result {
                Ok(ws) => {
                    path = ws.path.display().to_string();
                    if !ws.path.exists() {
                        moved = format!("  {} path has moved", "warning:".bold().yellow());
                    }
                }
                Err(error) => {
                    path = format!("{} {}", "warning:".bold().yellow(), error.cause());
                }
            }
            Row::new(vec![Cell::new(name), Cell::new(&path), Cell::new(&moved)])
        })
        .collect();
    let mut table = Table::init(rows);
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.printstd();
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

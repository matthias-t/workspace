#[macro_use]
mod macros;
mod app;
mod exit;
mod shell;
mod tilde;
mod workspace;

use clap::ArgMatches;
use colored::Colorize;
use failure::Fail;

use std::env;
use std::fs;
use std::io::Write;
use std::path;
use std::process;

use crate::exit::Exit;
use crate::tilde::Tilde;
use crate::workspace::Workspace;

pub static mut VERBOSE: bool = false;

fn main() {
    let matches = app::cli().get_matches();

    unsafe {
        VERBOSE = matches.is_present("verbose");
    }

    if !matches.is_present("shell-wrapper") && matches.subcommand_matches("shell").is_none() {
        warn!("The workspace binary is the backend for the `ws` function.");
        indent_warn!(
            "To set it up in your shell, see the README.md or run 'workspace shell --help'"
        )
    }

    match matches.subcommand() {
        ("open", Some(matches)) => {
            let name: &str = matches.value_of("NAME").unwrap();
            let ws = Workspace::get(name)
                .unwrap_or_exit(&format!("A workspace called '{}' does not exist", name))
                .unwrap_or_else(|error| {
                    let path = Workspace::file_path(name);
                    error!("{} from {}", error, path.tilde_format());
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
                indent_error!("the path '{}' was moved or deleted", ws.path.tilde_format());
                process::exit(1);
            }
            let dir_only = matches.is_present("directory");
            ws.open(dir_only);
        }

        ("add", Some(matches)) => {
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
                }).collect();

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

            let ws = Workspace {
                path,
                commands: workspace::Commands::default(),
                tabs: Vec::default(),
            };
            ws.write(&name);
            Workspace::edit(&name);
            println!("Created workspace '{}' in {}", name, ws.path.tilde_format());
        }

        ("edit", Some(matches)) => {
            let name = matches.value_of("NAME").unwrap();
            if !Workspace::exists(&name) {
                error!("A workspace called '{}' does not exist", name);
                process::exit(1);
            }
            Workspace::edit(name);
        }

        ("rename", Some(matches)) => {
            let old_name = matches.value_of("OLD_NAME").unwrap();
            let new_name = matches.value_of("NEW_NAME").unwrap();
            if !Workspace::exists(&old_name) {
                error!("A workspace called '{}' does not exist", old_name);
                process::exit(1);
            }
            if Workspace::exists(&new_name) {
                error!(
                    "Cannot rename to '{}' because a workspace with that name already exists",
                    new_name
                );
                process::exit(1)
            }
            std::fs::rename(
                Workspace::file_path(old_name),
                Workspace::file_path(new_name),
            ).unwrap_or_exit("Could not rename config file");
        }

        ("delete", Some(matches)) => {
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

        ("list", Some(_)) => {
            let all = Workspace::all();
            if all.is_empty() {
                println!("No existing workspaces.\nRun `ws add <NAME>` to create one.");
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
                        path = ws.path.tilde_format().bright_black().to_string();
                        if !ws.path.exists() {
                            moved = format!("{} path has moved", "warning:".bold().yellow());
                        }
                    }
                    Err(error) => {
                        path = format!("{} {}", "warning:".bold().yellow(), error);
                    }
                }
                let name =
                    name.unwrap_or_else(|| format!("{} invalid UTF-8", "warning:".bold().yellow()));

                grid.add(name.into());
                grid.add(path.into());
                grid.add(moved.into());
            }
            print!("{}", grid.fit_into_columns(3));
        }

        ("shell", Some(matches)) => {
            if matches.subcommand_matches("bash").is_some() {
                println!("{}", shell::BASH);
            } else if matches.subcommand_matches("fish").is_some() {
                println!("{}", shell::FISH);
            } else if matches.subcommand_matches("powershell").is_some() {
                println!("{}", shell::POWERSHELL)
            } else if let Some(matches) = matches.subcommand_matches("cmd") {
                let path: path::PathBuf = path_to_binary_or_arg(&matches);
                let mut file: fs::File = fs::OpenOptions::new()
                    .read(false)
                    .write(true)
                    .create(true)
                    .append(false)
                    .truncate(true)
                    .open(&path)
                    .unwrap_or_exit(&format!(
                        "Could not create batch file at {}",
                        path.tilde_format()
                    ));

                file.write_fmt(format_args!("{}", shell::CMD))
                    .unwrap_or_exit("Could not write to batch file");

                println!("Wrote {}", path.tilde_format());
            }
        }

        _ => {}
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

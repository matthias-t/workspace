#[macro_use]
pub mod macros;
mod shell;
mod workspace;

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate colored;

use clap::*;
use colored::*;
use std::env;
use std::fs;
use std::io::Write;
use std::path;
use std::process;
use workspace::Workspace;

fn main() {
    let matches = App::new("workspace")
        .version("0.0.0")
        .about("Manages workspaces for all your projects!")
        .author("Matthias T. and Roma B.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("open")
                .about("Opens a workspace")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to open")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new workspace in this directory")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the new workspace")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a specified workspace, if present")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to delete")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("Lists all existing workspaces"),
        )
        .subcommand({
            SubCommand::with_name("shell")
                .about("Sets up workspace in your shell")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("bash")
                        .about("Returns a bash function to source in your bashrc")
                        .long_about(
                            "Returns a bash function to source in your bashrc with \nsource <(workspace shell bash)"
                        ),
                )
                .subcommand(
                    SubCommand::with_name("powershell")
                        .alias("PowerShell")
                        .alias("posh")
                        .about("Returns a PowerShell function to source in your shell profile")
                        .long_about(
                            "Returns a PowerShell function to source in your shell profile with \nInvoke-Expression \"$(workspace shell powershell)\""
                        ),
                )
                .subcommand(
                    SubCommand::with_name("cmd")
                        .about("Creates a cmd batch file")
                        .long_about(
                            "Creates a cmd batch file. Unless PATH is specified, it will be created in the same folder as the workspace binary",
                        )
                        .arg(Arg::with_name("PATH")),
                )
        })
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("open") {
        open(matches);
    } else if let Some(matches) = matches.subcommand_matches("new") {
        new(matches);
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
    let ws: Workspace = workspace::get(name).unwrap_or_else(|| {
        eprintln!("ERROR: A workspace called '{}' does not exist", name);
        std::process::exit(1);
    });
    ws.cd();
}

fn new(matches: &ArgMatches) {
    let ws = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().expect("ERROR: Could not read current directory"),
    };
    if ws.exists() {
        eprintln!("ERROR: A workspace called '{}' already exists", ws.name);
        std::process::exit(1);
    }
    ws.write();
    println!("Created workspace '{}' in {}", ws.name, ws.path.display());
}

fn delete(matches: &ArgMatches) {
    let ws = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().expect("ERROR: Could not read current directory"),
    };
    if !ws.exists() {
        eprintln!("ERROR: A workspace called '{}' does not exist", ws.name);
        process::exit(1);
    }
    ws.delete();
    println!("Deleted workspace '{}' in {}", ws.name, ws.path.display());
}

fn list() {
    let all = workspace::all();

    if all.is_empty() {
        println!("No existing workspaces.\nRun `workspace new <NAME>` to create one.");
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
            .unwrap_or_else(|_| {
                eprintln!("ERROR: Could not create batch file at {}", path.display());
                process::exit(1);
            });

        file.write_fmt(format_args!("{}", shell::CMD))
            .expect("Could not write to batch file");

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
        let mut path = env::current_exe().unwrap_or_else(|_| {
            eprintln!("ERROR: Could not determine path to binary");
            println!("Try providing a PATH");
            process::exit(1);
        });
        path.set_file_name("ws");
        path.set_extension("bat");
        return path;
    }
}

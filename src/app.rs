use clap::*;

pub fn cli() -> App<'static, 'static> {
    App::new("workspace")
        .version("0.3.0")
        .about("A command-line project manager")
        .author("Matthias T. and Roma B.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::ColorAlways)
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Causes verbose output to be logged"),
        )
        .arg(
            Arg::with_name("shell-wrapper")
                .long("--from-shell-wrapper")
                .hidden(true)
        )
        .subcommand(
            SubCommand::with_name("open")
                .about("Opens a workspace")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to open")
                        .required(true),
                )
                .arg(
                    Arg::with_name("directory")
                        .help("Only change the directory")
                        .short("d")
                        .long("directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .alias("new")
                .about("Creates a new workspace in this directory")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the new workspace")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edits a workspace")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to edit")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rename")
                .alias("mv")
                .about("Renames a workspace")
                .arg(
                    Arg::with_name("OLD_NAME")
                        .help("Name of the workspace to rename")
                        .required(true),
                )
                .arg(
                    Arg::with_name("NEW_NAME")
                        .help("New name of the workspace")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .alias("remove")
                .alias("rm")
                .about("Deletes a workspace")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to delete")
                        .required(true),
                )
                .arg(
                    Arg::with_name("yes")
                        .long("yes")
                        .short("y")
                        .help("Skips confirmation prompt"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("Lists all workspaces"),
        )
        .subcommand({
            SubCommand::with_name("shell")
                .about("Sets up `ws` in your shell")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("bash")
                        .about("Returns a bash function to source in your bashrc")
                        .long_about(
                            "Returns a bash function to source in your bashrc with \nsource <(workspace shell bash)"
                        ),
                )
                .subcommand(
                    SubCommand::with_name("fish")
                        .about("Returns a fish function to source in your fish.config")
                        .long_about(
                            "Returns a fish function to source in your fish.config with \nworkspace shell fish | source"
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
}

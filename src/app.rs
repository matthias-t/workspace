use clap::*;

pub fn cli() -> App<'static, 'static> {
    App::new("workspace")
        .version("0.0.0")
        .about("Manages workspaces for all your projects!")
        .author("Matthias T. and Roma B.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Causes verbose output to be logged"),
        )
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
            SubCommand::with_name("delete")
                .alias("remove")
                .alias("rm")
                .about("Deletes a specified workspace, if present")
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

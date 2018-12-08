## `0.4.0`
- switch to `YAML`
- add documentation about workspace format
- `ws add` doesn't generate empty lists and tables anymore
- improve some help messages
- print warnings to `stderr`
- show a warning when the binary is used directly
- improve code style, modernize imports

## `0.3.0`
- instead of the shell wrappers, `std::process` is now used to open tabs or run commands in a new terminal
- new `commands.background` field: each background command is the argument of a new shell process
- updated the project description

## `0.2.2`
- paths above home directory are now shortened with a tilde `~`
- updated the project description and README.md

## `0.2.1`
fix a bug that made the `add` subcommand panic

## `0.2.0`
- new workspace field: `tabs`, a list of strings to be opened with `$BROWSER`
- `commands` is now `commands.local` and there is a new `commands.external` field

## `0.1.0`
initial release

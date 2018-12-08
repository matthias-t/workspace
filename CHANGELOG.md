## [`0.4.0`](48bd22e8079f0ea32a0a3127c37294c1fd9bab51)
- switch to `YAML`
- add documentation about workspace format
- error on unknown workspace fields
- `ws add` doesn't generate empty lists and tables anymore
- improve some help messages
- print warnings to `stderr`
- show a warning when the binary is used directly
- improve code style, modernize imports

## [`0.3.0`](7b09d1948816439b7c598f92dc0535f0b2ab101a)
- instead of the shell wrappers, `std::process` is now used to open tabs or run commands in a new terminal
- new `commands.background` field: each background command is the argument of a new shell process
- updated the project description

## [`0.2.2`](a5d0aad79c12a809cbab90bddbf5155aac526d7d)
- paths above home directory are now shortened with a tilde `~`
- updated the project description and README.md

## [`0.2.1`](fc4532683b6be21cd51efe3596aa64e4132136e1)
fix a bug that made the `add` subcommand panic

## [`0.2.0`](d380b6924e4df26cf85ff8e842d95b1b2c2f0ce8)
- new workspace field: `tabs`, a list of strings to be opened with `$BROWSER`
- `commands` is now `commands.local` and there is a new `commands.external` field

## [`0.1.0`](1ace6469b076889a7114484f56724fdd533585c2)
initial release

# workspace [![Build Status](https://travis-ci.com/matthias-t/workspace.svg?branch=master)](https://travis-ci.com/matthias-t/workspace)

`ws` is a CLI to manage and interpret small YAML files that specify tasks to open a project like opening an editor, launching a server or visiting a chat or documentation in the browser. It can be used to efficiently switch between work and side projects.

## Installation

```bash
cargo install workspace
```

Then setup the `ws` command in your shell:

- **bash**: Add this line to your `.bashrc`
  ```bash
  eval $(workspace shell bash)
  ```
- **fish**: Add this line to your `config.fish`
  ```fish
  workspace shell fish | source
  ```
- **PowerShell**: Add this line to your `profile.ps1`
  ```powershell
  Invoke-Expression "$(workspace shell posh)"
  ```

> `workspace shell` prints a shell function `ws` that delegates output from `workspace` but intercepts commands to run. This lets you change the directory and run commands directly in the shell, e.g. if they need user input.

## Documentation

For the CLI, see:
```
ws --help
```

Workspaces can have the following fields:

- `path`, list of strings <br>
  path to the workspace
- `tabs`, list of strings <br>
  tabs to open in `$BROWSER`
- `commands`, table
  - `local`, list of strings <br>
    commands execute in the current shell
  - `background`, list of strings <br>
    commands execute as background processes
  - `external`, list of strings <br>
    commands to execute in a new `$TERMINAL`

> Note: `path` is mandatory and created automatically by `ws new`

For example, this is the workspace I use for my blog:
```
path: /home/matthias/code/web/blog/

commands:
  local:
  - git status
  - sudo systemctl start nginx
  background:
  - code -r .
  external:
  - gulp

tabs:
- https://developer.mozilla.org/en-US/
- localhost
```
It will `cd` into `~/code/web/blog/`, print the git status, open the directory in visual studio code, start the `gulp` build in a new terminal, launch `nginx` to serve the files and open `localhost` and MDN in the browser.

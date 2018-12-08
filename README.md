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

## Documentation

For the CLI, see:
```
ws --help
```

Workspaces are YAML files. They can have the following fields:
- `path`, string: path to the workspace
- `tabs`, list of strings: tabs to open in `$BROWSER`
- `commands`, table
  - `local`, list of strings: commands to be ran in the current shell
  - `background`, list of strings: commands to be ran in a new background process
  - `external`, list of strings: commands to be ran in a new `$TERMINAL`

> Note: `path` is mandatory and created automatically by `ws new`

For example, this is the workspace I use for my blog:
```
path: /home/matthias/code/web/blog/

tabs:
- https://developer.mozilla.org/en-US/
- localhost

commands:
  local:
  - git status
  background:
  - sudo systemctl start nginx
  - code -r .
  external:
  - gulp
```
It will `cd` into `~/code/web/blog/`, print the git status, open the directory in visual studio code, start the `gulp` build in a new terminal, launch `nginx` to serve the files and open `localhost` and MDN in the browser.

## FAQ

> Should I use `workspace` or `ws`?

Use `ws`. `workspace` is the binary that powers the `ws` function and sets it up in your shell configuration.

> Why do I need to add something to my shell configuration?

Otherwise workspace can't change your working directory or run commands that you specify for a workspace directly in the shell process.

> I don't trust you

That's not technically a question. But the good thing is: you don't need to. If you run `workspace shell ...` you can see what you are invoking. Or you could just take a look at the code.

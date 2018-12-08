# workspace [![Build Status](https://travis-ci.com/matthias-t/workspace.svg?branch=master)](https://travis-ci.com/matthias-t/workspace)

`ws` is a CLI to manage and interpret small YAML files that specify tasks to open a project, like opening an editor, launching a server or visiting slack or documentation in the browser. For example, it can be used to efficiently switch between work and side projects.

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

```
ws --help
```

More documentation is coming about the format of workspace files when it becomes more stable. For now, use the outline created by `ws add`.

## FAQ

> Should I use `workspace` or `ws`?

Use `ws`. `workspace` is the binary that powers the `ws` function and sets it up in your shell configuration.

> Why do I need to add something to my shell configuration?

Otherwise workspace can't change your working directory or run commands that you specify for a workspace directly in the shell process.

> I don't trust you

That's not technically a question. But the good thing is: you don't need to. If you run `workspace shell ...` you can see what you are invoking. Or you could just take a look at the code.

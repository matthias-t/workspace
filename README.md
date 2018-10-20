# workspace [![Build Status](https://travis-ci.com/matthias-t/workspace.svg?branch=master)](https://travis-ci.com/matthias-t/workspace)
> a CLI that takes care of your windows, tabs and working directory

## Installation

```bash
> cargo install workspace
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

## FAQ

> Should I use `workspace` or `ws`?

Use `ws`. `workspace` is the binary that powers the `ws` function and sets it up in your shell configuration.

> Why do I need to add something to my shell configuration?

Otherwise workspace can't change your working directory or run commands that you specify for a workspace directly in the shell process.

> I don't trust you

That's not technically a question. But the good thing is: you don't need to. If you run `workspace shell ...` you can see what you are invoking. Or you could just take a look at the code.

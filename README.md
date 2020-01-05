[![Build Status](https://travis-ci.org/nicolas-yangli/nvim_windows_remote.svg?branch=master)](https://travis-ci.org/nicolas-yangli/nvim_windows_remote)

Edit a file in an existing neovim instance.

# Usage

```
nvim_windows_remote

USAGE:
    nvim_windows_remote [OPTIONS] [file]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --edit-command <edit_command>     [possible values: edit, split, vsplit]

ARGS:
    <file>
```

## Add to Context Menu

1. Edit `editWithNeoVim.reg`.
    * Replace `C:\\Tools\\Neovim\\bin\\nvim-qt.exe`, `C:\\Tools\\nvim_windows_remote\\nvim_windows_remote.exe` with actual paths of executables.
    * (Optioanlly) Change edit command, `vsplit`, to whatever you want.
2. Import it to the registry.

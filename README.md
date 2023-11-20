# AxBind

> Dotfile management for the indecisive.

#### Note:

*Eventually will have binary/package posted, but as of right now `axbind` must be built from source.*

## Description

`axbind` is program intended to automate the task of editing multiple dotfiles when making a single abstract change to keybindings/colors/etc.


## Getting Started

Download the contents of the [example config directory] and place them in your preferred config directory (ex: ~/.config/axbind).

`axbind` by default will check these paths in order for a `master config` file when run:
1. ```$XDG_CONFIG_HOME/axbind/config.toml```
2. ```$HOME/.config/axbind/config.toml```
3. `/etc/axbind/config.toml`

*This behavior can be overriden by specifying `--config=<path>` with the `axbind` command.*

All `axbind` configuration files are parsed as [toml].




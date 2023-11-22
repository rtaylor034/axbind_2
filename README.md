# AxBind

> Dotfile management for the indecisive.

#### Note:

*Eventually will have binary/package posted, but as of right now `axbind` must be built from source.*

## Description

AxBind is program intended to automate the task of editing multiple dotfiles when making a single abstract change to keybindings/colors/etc.

This is achieved through user-defined maps of key-value pairs that are "applied" to all/specified text files of a directory (ex: ~/.config).

Beyond basic key-value replacement, mapping behavior can further be manipulated per-file (i.e. remapping, text functions, layering multiple maps, etc.).

AxBind is written in Rust and all configuration files of AxBind use [toml] syntax.

## Master Config

By default, AxBind will check these paths (in order) for a `master config` file when run:
1. `$XDG_CONFIG_HOME/axbind/config.toml`
2. `$HOME/.config/axbind/config.toml`
3. `/etc/axbind/config.toml`

*This behavior can be overriden by specifying `--config=<path>` with the `axbind` command.*

#### Required Values
| Name | Type | Description |
| `map_directory` | String | . |
| `function_directory` | String | . |
| `tag_directory` | String | . |


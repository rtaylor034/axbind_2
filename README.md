# AxBind

> Dotfile management for the indecisive.

#### Note:

*Eventually will have binary/package posted, but as of right now `axbind` must be built from source.*

## Description

AxBind is program intended to automate the task of editing multiple dotfiles when making a single abstract change to keybindings/colors/etc.

This is achieved through user-defined mappings that are applied to all/specified text files of a directory (ex: ~/.config).

Beyond basic key-value replacement, mapping behavior can be further controlled per-file (i.e. remapping, text functions, layering multiple maps, etc.).

AxBind is written in Rust and all configuration files of AxBind use [toml] syntax.

# Documentation

For quickly getting started, an [example configuraton] is provided with reasonable defaults and guiding comments.

## Master Config File

By default, AxBind will check these paths (in order) for a [Master Config File]:
1. `$XDG_CONFIG_HOME/axbind/config.toml`
2. `$HOME/.config/axbind/config.toml`
3. `/etc/axbind/config.toml`

This behavior can be overriden by specifying `--config=<path>` with the `axbind` command.

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `map_directory` | String | . |
| `function_directory` | String | . |
| `tag_directory` | String | . |
| `tag_entry_point` | String | . |
| `options.meta` | [Meta Options] | . |
| `options.group` | [Group Options] | . |
| `options.layer` | [Group Options] | . |

#### Example:
```toml
# .../config.toml
map_directory = 'maps'
function_directory = 'functions'
tag_directory = '.axbind'
tag_entry_point = 'main.toml'

[options.meta]
escape_sequence = '|'
wildcard_character = '^'

[options.group]
axbind_file_format = '^.axbind'

[options.layer]
escape_sequence = '|'
key_format = '@^@'
```

### Map/Function Directory

### Tag Directory


## Map File

Represents a [Mapping] and its name.

*AxBind will ignore files within the map directory that are not valid toml, or do not contain an `axbind_map` key.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_map` | String | . |
| `map` | [Mapping] | . |

#### Example:
```toml
# ~/.config/axbind/maps/myMap.toml
axbind_map = 'myMap'
[map]
foo = 'myFooReplacement'
bar = 'myBarReplacement'
```

## Map

Represents a user-specified mapping of key-value pairs.

*The important part of a [Map File].*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| \<any> | String | . |
| `@INCLUDE` | String[] [?] | . |

#### Example
```toml
#<...>
foo = 'myFooReplacement'
bar = 'myBarReplacement'
#<...>
```

## Function File

Represents a [Function] and its name.

*AxBind will ignore files within the function directory that are not valid toml, or do not contain an `axbind_function` key.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_function` | String | . |
| `function` | [Function] | . |

#### Example:
```toml
# ~/.config/axbind/functions/myFunction.toml
axbind_map = 'myFunction'
[function]
shell = 'sh'
command = 'echo -n "This used to be ^"'
```

## Function

Represents a user-specified string-to-string function.

*The important part of a [Function File].*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `shell` | String | . |
| `command` | String | . |

#### Example
```toml
#<...>
shell = 'sh'
command = 'echo -n "This used to be ^"'
#<...>
```

## Tag Entry Point

AxBind expects a [Tag Entry Point] to be present in every [tag directory] and reads it first.

*Currently, [Tag Entry Points] only exist to specify [Tag Groups], if any.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `groups`* | String[] [?] | . |

\*If unspecified, AxBind will treat the [Tag Entry Point] file itself as a [Tag Group] (and assume it is the only one), and will read it as such.

#### Example:

```toml
# ~/.config/lf/.axbind/main.toml
groups = [
  'colors.toml'
  'keybindings.toml'
]
```

## Tag Group

Tells AxBind which files to apply [Layers] to and how.

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `files` | String[] | . |
| `options` | [GroupOptions] | . |
| `layers` | [Layer][] | . |

#### Example:
```toml
# ~/.config/lf/.axbind/keybindings.toml
files = [
  'lfrc'
]
options.axbind_file_format = '^.myExtension'
[[layers]]
map = 'myKeybindings'
remaps = [ 'myLfRemaps' ]
```

## Layer

Represents a 'pass' of AxBind.

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `map` | String | . |
| `remaps` | String[] | . |
| `functions` | String[] | . |
| `options` | [LayerOptions][] [?] | . |

#### Example:
```toml
#<...>
map = 'myKeybindings'
remaps = [ 'myLfRemaps' ]
#<...>
```

## Meta Options

These options relate to how AxBind reads its own configuration files.

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence`[^opt] | String | . |
| *`wildcard_character`[^opt] | Single Character | . |

#### Example:
```toml
#<...>
escape_sequence = '|'
wildcard_char = '^'
#<...>
```

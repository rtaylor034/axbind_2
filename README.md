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

**For quickly getting started, an [example configuraton] is provided with reasonable defaults and guiding comments.**

### About The Docs

The following sections describe AxBind configuration files/types in a sensible order.

Becuase all configuration files are just [toml] tables, they can be described with their expected keys as shown:

| Key | Type | Description |
|:----|:-----|:------------|
| `example_key` | Example [?] | Description of what the value for this key represents. |

If the 'Type' is marked with a [?] as shown above, specifying the key-value pair is optional.

All other key-value pairs are required to be specified, and AxBind will panic or skip the file (depending on the file) if they are not.

## Master Config File

By default, AxBind will check these paths (in order) for a [Master Config File]:
1. `$XDG_CONFIG_HOME/axbind/config.toml`
2. `$HOME/.config/axbind/config.toml`
3. `/etc/axbind/config.toml`

This behavior can be overriden by specifying `--config=<path>` with the `axbind` command.

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `map_directory` | String | Path that AxBind reads for [Map Files], relative to this [Master Config File]'s directory.  |
| `function_directory` | String | Path that AxBind reads for [Function Files], relative to this [Master Config File]'s directory. |
| `tag_directory` | String | Path of directory that AxBind recursively searches for inside the specified [Root Directory]. Matching paths are deemed 'tag directories', and the directory *containing* them are "tagged" for AxBind modification. |
| `tag_entry_point` | String | Path to the [Tag Entry Point File] in each 'tag directory'. |
| `options.meta` | [Meta Options] | Default [Meta Options]. *Currently the only place where [Meta Options] are specified.* |
| `options.group` | [Group Options] | Default [Group Options] if they are unspecified in a [Tag Group File]. |
| `options.layer` | [Layer Options] | Default [Layer Options] if they are unspecified in a [Layer]. |

#### Example:
```toml
# .../config.toml
map_directory = 'maps'
function_directory = 'functions'
tag_directory = '.axbind'
tag_entry_point = 'main.toml'

[options.meta]
escape_sequence = '|'
wildcard_char = '^'

[options.group]
axbind_file_format = '^.axbind'

[options.layer]
escape_sequence = '|'
key_format = '@^@'
```

## Map File

Represents a [Map] and its name.

*AxBind will ignore (but will warn about) files within the map directory that are not valid toml, or do not contain an `axbind_map` key.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_map` | String | Name of the [Map] represented. |
| `map` | [Map] | [Map] being represented. |

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

Component of [Map File].

*The name of a [Map] is specified in its representative [Map File].*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| \<any> | String | Key-value pairs that make up this [Map]. |
| `@INCLUDE` | String[] [?] | List of [Map] names; This [Map] will inherit all key-value pairs of the [Maps] specified, in-order ([Maps] specified last will override duplicate keys). Key-value pairs directly specified in this [Map] *(are supposed too [See [Known Issues]])* override included [Maps]. |

#### Example
```toml
#<...>
foo = 'myFooReplacement'
bar = 'myBarReplacement'
#<...>
```

## Function File

Represents a [Function] and its name.

*AxBind will ignore (but will warn about) files within the function directory that are not valid toml, or do not contain an `axbind_function` key.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_function` | String | Nme of the [Function] represented. |
| `function` | [Function] | [Function] being represented. |

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

Can be thought of as a dynamic [Map] that generates its key-value pairs based on a shell script/command.

Can only be used to \*remap\* values.

Component of [Function File].

*The name of a [Function] is specified in its representative [Function File].*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `shell` | String | Shell executable/command that runs the body command. *The function is executed as `<shell> -c "<command>"`*. |
| `command` | [KeyString] | Shell command; [Function] body. The [wildcard] is replaced with the unmapped value (input), and the standard out is the output of this [Function]. |

#### Example
```toml
#<...>
shell = 'sh'
command = 'echo -n "This used to be ^"'
#<...>
```

## Tag Entry Point File

AxBind expects a [Tag Entry Point File] to be present in every [tag directory] and reads it first.

*Currently, [Tag Entry Point Files] only exist to specify [Tag Group Files], if any.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `groups`* | String[] [?] | List of paths to [Tag Group Files] relative to the 'tag directory'. [Tag Group Files] are evaluated in the order specified. |

\*If unspecified, AxBind will treat the [Tag Entry Point File] file itself as a [Tag Group File] (and assume it is the only one), and will read it as such.

#### Example:

```toml
# ~/.config/lf/.axbind/main.toml
groups = [
  'colors.toml',
  'keybindings.toml'
]
```

## Tag Group File

Tells AxBind which files to apply specified [Layers] to.

*If two or more [Tag Group Files] affect the same file, the file will only hold the result of the last [Tag Group File] evaluated.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `files` | String[] | List of file paths relative to the \*tagged\* directory that this group affects. Specified file paths will be *written* to after reading each files respective 'AxBind file' (See [Group Options]). |
| `options` | [Group Options] [?] | This group's options. Overrides the defaults specified in the [Master Config File]. |
| `layers` | [Layer][] | [Layers] to apply—in order—to all axbind files that this group affects. [Layers] are applied one-after-another and will read the output of the previous. |

#### Example:
```toml
# ~/.config/lf/.axbind/keybindings.toml
files = [
  'lfrc'
]
options.axbind_file_format = '^.myCustomExtension'
[[layers]]
map = 'myKeybindings'
remaps = [ 'myLfRemaps' ]
options.key_format = '%^%'
```

## Layer

Represents a specification for AxBind mapping.

Component of [Tag Group File].

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `map` | String | [Map] name; all instances of this [Map]'s keys in the specified [key format] will be replaced with its respective value (after specified `remaps` and `functions` are applied to it) when this [Layer] is applied. |
| `remaps` | String[] | List of [Map] names; each value of `map` will be re-mapped (values used as keys) by these [Maps], one-after-another in-order. |
| `functions` | String[] | List of [Function] names; each value of `map` will be modified by these [Functions], one-after-another in-order. *`functions` are applied *after* all remaps (see [Known Issues]).* |
| `options` | [Layer Options][] [?] | This layer's options. Overrides the defaults specified in the [Master Config File].  |

#### Example:
```toml
#<...>
map = 'myKeybindings'
remaps = [ 'myLfRemaps' ]
options.key_format = '%^%'
#<...>
```

## Meta Options

Options relating to how AxBind reads its own configuration files.

Component of [Master Config File].

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence` | String [?] | [Escape sequence] that can be used in all interpreted [Key Strings]. *Currently is only useful for escaping the `wildcard_char`.* |
| `wildcard_char` | String [?] | Must be a single character; character representing the arbitrary input/value in a [Key String], replaced with an actual value at evaluation depending on the context. *(See [Key String])* |

#### Example:
```toml
#<...>
escape_sequence = '|'
wildcard_char = '^'
#<...>
```

### Key String

A String that replaces the any instance of the current `wildcard_char` (referred to as just the 'wildcard') with another value when evaluated.

wildcards can be [escaped].

### Escape Sequence

A character sequence that tells AxBind to treat the character directly after the sequence as a non-special character (regardless of if it is or not) when evaluating text.

*(I.e. allows for instances of wildcard characters or map keys to exist in evaluated outputs.)*

Escape sequences are removed from the evaluated output unless they themselves are escaped.

#### Example:
> Given this [Map File]:
```toml
axbind_map = 'foobar'
[map]
foo = 'BAR'
```
> Applying this [Layer]:
```toml
map = 'foobar'
key_format = '@^@'
escape_sequence = '|'
```
> To this text:
```
1 @foo@
2 |@foo@
3 @foo@|
4 @f|oo@
5 ||@foo@
6 @fo|Ao@
se|c|ret| me|ssa|ge
```
> Yields the following:
```
1 BAR
2 @foo@
3 BAR
4 @foo@
5 |BAR
6 @foAo@
secret message
```

## Group Options

Options relating to [Tag Group] specifications.

Component of [Tag Group], [Master Config File].

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_file_format` | [Key String] [?] | Format of each file's respective 'AxBind file'. the wildcard represents the name of the file being written too. *(See [Tag Group].`files`)* |

#### Example:
```toml
#<...>
axbind_file_format = '^.myCustomExtension'
#<...>
```

## Layer Options

Options relating to how [Layers] behave.

Component of [Layer], [Master Config File].

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence` | String [?] | [Escape sequence] that can be used to escape keys in 'AxBind files' that the [Layer] affects. |
| `key_format` | [Key String] [?] | Format that keys must be in to be recognized by the [Layer] in an 'AxBind file'. the wildcard represents the raw key of the primary [Map]. |

#### Example:
```toml
#<...>
options.key_format = '%^%'
#<...>
```

# Known Issues

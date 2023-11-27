# AxBind

> Dotfile management for the indecisive.

#### Note:

*Eventually will have binary/package posted, but as of right now `axbind` must be built from source.*

## Description

AxBind is program intended to automate the task of editing multiple dotfiles when making a single abstract change to keybindings/colors/etc.

This is achieved through user-defined mappings that are applied to all/specified text files of a directory (ex: ~/.config).

Beyond basic key-value replacement, mapping behavior can be further controlled per-file (i.e. remapping, text functions, layering multiple maps, etc.).

AxBind is written in Rust and all configuration files of AxBind use [toml](https://toml.io/en/) syntax.

# Documentation

**For quickly getting started, an [example configuraton] is provided with reasonable defaults and guiding comments.**

### About The Docs

The following sections describe AxBind configuration files/types in a sensible order.

Becuase all configuration files are just [toml](https://toml.io/en/) tables, they can be described with their expected keys as shown:

| Key | Type | Description |
|:----|:-----|:------------|
| `example_key` | Example **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Description of what the value for this key represents. |

Specifying the key-value pair is optional if the 'Type' is marked with a **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** as shown above.

All other key-value pairs are required to be specified, and AxBind will panic or ignore the entire object (depending on the context) if they are not.

## Master Config File

By default, AxBind will check these paths (in order) for a Master Config File:
1. `$XDG_CONFIG_HOME/axbind/config.toml`
2. `$HOME/.config/axbind/config.toml`
3. `/etc/axbind/config.toml`

This behavior can be overriden by specifying `--config=<path>` with the `axbind` command.

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `map_directory` | String | Path that AxBind reads for [Map Files](https://github.com/rtaylor034/axbind_2#map-file), relative to this Master Config File's directory.  |
| `function_directory` | String | Path that AxBind reads for [Function Files](https://github.com/rtaylor034/axbind_2#function-file), relative to this Master Config File's directory. |
| `tag_directory` | String | Path of directory that AxBind recursively searches for inside the specified [Root Directory]. Matching paths are deemed 'tag directories', and the directory *containing* them are "tagged" for AxBind modification. |
| `tag_entry_point` | String | Path to the [Tag Entry Point File](https://github.com/rtaylor034/axbind_2#tag-entry-point-file) in each 'tag directory'. |
| `options.meta` | [Meta Options](https://github.com/rtaylor034/axbind_2#meta-options) | Default [Meta Options](https://github.com/rtaylor034/axbind_2#meta-options). *Currently the only place where [Meta Options](https://github.com/rtaylor034/axbind_2#meta-options) are specified.* |
| `options.group` | [Group Options](https://github.com/rtaylor034/axbind_2#group-options) | Default [Group Options](https://github.com/rtaylor034/axbind_2#group-options) if they are unspecified in a [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file). |
| `options.layer` | [Layer Options](https://github.com/rtaylor034/axbind_2#layer-options) | Default [Layer Options](https://github.com/rtaylor034/axbind_2#layer-options) if they are unspecified in a [Layer](https://github.com/rtaylor034/axbind_2#layer). |

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

Represents a [Map](https://github.com/rtaylor034/axbind_2#map) and its name.

*AxBind will ignore (but will warn about) files within the map directory that are not valid toml, or do not contain an `axbind_map` key.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_map` | String | Name of the [Map](https://github.com/rtaylor034/axbind_2#map) represented. |
| `map` | [Map](https://github.com/rtaylor034/axbind_2#map) | [Map](https://github.com/rtaylor034/axbind_2#map) being represented. |

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

Component of [Map File](https://github.com/rtaylor034/axbind_2#map-file).

*The name of a Map is specified in its representative [Map File](https://github.com/rtaylor034/axbind_2#map-file).*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| \<any> | String | Key-value pairs that make up this Map. |
| `@INCLUDE` | String[] **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | List of Map names; This Map will inherit all key-value pairs of the Maps specified, in-order (Maps specified last will override duplicate keys). Key-value pairs directly specified in this Map *(are supposed too [See [Known Issues](https://github.com/rtaylor034/axbind_2#known-issues)])* override included Maps. |

#### Example
```toml
#<...>
foo = 'myFooReplacement'
bar = 'myBarReplacement'
#<...>
```

## Function File

Represents a [Function](https://github.com/rtaylor034/axbind_2#function) and its name.

*AxBind will ignore (but will warn about) files within the function directory that are not valid toml, or do not contain an `axbind_function` key.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_function` | String | Nme of the [Function](https://github.com/rtaylor034/axbind_2#function) represented. |
| `function` | [Function](https://github.com/rtaylor034/axbind_2#function) | [Function](https://github.com/rtaylor034/axbind_2#function) being represented. |

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

Can be thought of as a dynamic [Map](https://github.com/rtaylor034/axbind_2#map) that generates its key-value pairs based on a shell script/command.

Can only be used to \*remap\* values.

Component of [Function File](https://github.com/rtaylor034/axbind_2#function-file).

*The name of a Function is specified in its representative [Function File](https://github.com/rtaylor034/axbind_2#function-file).*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `shell` | String | Shell executable/command that runs the body command. *The function is executed as `<shell> -c "<command>"`*. |
| `command` | [Key String](https://github.com/rtaylor034/axbind_2#key-string) | Shell command; Function body. The wildcard represents an unmapped value (input), and the standard out is the output of this Function. |

#### Example
```toml
#<...>
shell = 'sh'
command = 'echo -n "This used to be ^"'
#<...>
```

## Tag Entry Point File

AxBind expects a Tag Entry Point File to be present in every [tag directory] and reads it first.

*Currently, Tag Entry Point Files only exist to specify [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file), if any.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `groups`* | String[] **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | List of paths to [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file) relative to the 'tag directory'. [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file) are evaluated in the order specified. |

\*If unspecified, AxBind will treat the Tag Entry Point File file itself as a [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file) (and assume it is the only one), and will read it as such.

#### Example:

```toml
# ~/.config/lf/.axbind/main.toml
groups = [
  'colors.toml',
  'keybindings.toml'
]
```

## Tag Group File

Tells AxBind which files to apply specified [Layers](https://github.com/rtaylor034/axbind_2#layer) to.

*If two or more Tag Group Files affect the same file, the file will only hold the result of the last Tag Group File evaluated.*

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `files` | String[] | List of file paths relative to the \*tagged\* directory that this group affects. Specified file paths will be *written* to after reading each files respective 'AxBind file' (See [Group Options](https://github.com/rtaylor034/axbind_2#group-options)). |
| `options` | [Group Options](https://github.com/rtaylor034/axbind_2#group-options) **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | This group's options. Overrides the defaults specified in the [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file). |
| `layers` | [Layer](https://github.com/rtaylor034/axbind_2#layer)[] | [Layers](https://github.com/rtaylor034/axbind_2#layer) to apply—in order—to all axbind files that this group affects. [Layers](https://github.com/rtaylor034/axbind_2#layer) are applied one-after-another and will read the output of the previous. |

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

Component of [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file).

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `map` | String | [Map](https://github.com/rtaylor034/axbind_2#map) name; all instances of this [Map](https://github.com/rtaylor034/axbind_2#map)'s keys in the specified [key format](https://github.com/rtaylor034/axbind_2#layer-options) will be replaced with its respective value (after specified `remaps` and `functions` are applied to it) when this [Layer](https://github.com/rtaylor034/axbind_2#layer) is applied. |
| `remaps` | String[] | List of [Map](https://github.com/rtaylor034/axbind_2#map) names; each value of `map` will be re-mapped (values used as keys) by these [Maps](https://github.com/rtaylor034/axbind_2#map), one-after-another in-order. |
| `functions` | String[] | List of [Function](https://github.com/rtaylor034/axbind_2#function) names; each value of `map` will be modified by these [Functions](https://github.com/rtaylor034/axbind_2#function), one-after-another in-order. *`functions` are applied *after* all remaps (see [Known Issues](https://github.com/rtaylor034/axbind_2#known-issues)).* |
| `options` | [Layer Options](https://github.com/rtaylor034/axbind_2#layer-options)[] **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | This Layer's options. Overrides the defaults specified in the [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).  |

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

Component of [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence` | String **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | [Escape sequence](https://github.com/rtaylor034/axbind_2#escape-sequence) that is recognized by AxBind when interpreting [Key Strings](https://github.com/rtaylor034/axbind_2#key-string). *Currently is only useful for escaping the `wildcard_char`.* |
| `wildcard_char` | String **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Must be a single character; character representing the arbitrary input/value in a [Key String](https://github.com/rtaylor034/axbind_2#key-string), replaced with an actual value at evaluation depending on the context. *(See [Key String](https://github.com/rtaylor034/axbind_2#key-string))* |

#### Example:
```toml
#<...>
escape_sequence = '|'
wildcard_char = '^'
#<...>
```

### Key String

A String that replaces the any instance of the current `wildcard_char` (referred to as just the 'wildcard') with another value when evaluated.

wildcards can be [escaped](https://github.com/rtaylor034/axbind_2#escape-sequence).

### Escape Sequence

A character sequence that tells AxBind to treat the character directly after the sequence as a non-special character (regardless of if it is or not) when evaluating text.

*(I.e. allows for instances of wildcard characters or map keys to exist in evaluated outputs.)*

Escape sequences are removed from the evaluated output unless they themselves are escaped.

#### Example:
> Given this [Map File](https://github.com/rtaylor034/axbind_2#map-file):
```toml
axbind_map = 'foobar'
[map]
foo = 'BAR'
```
> Applying this [Layer](https://github.com/rtaylor034/axbind_2#layer):
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

Options relating to [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file) specifications.

Component of [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file), [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_file_format` | [Key String](https://github.com/rtaylor034/axbind_2#key-string) **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Format of each file's respective 'AxBind file'. the wildcard represents a name of a file the [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file) applies too. *(See [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file).`files`)* |

#### Example:
```toml
#<...>
axbind_file_format = '^.myCustomExtension'
#<...>
```

## Layer Options

Options relating to how [Layers](https://github.com/rtaylor034/axbind_2#layer) behave.

Component of [Layer](https://github.com/rtaylor034/axbind_2#layer), [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).

#### Checked Keys:
| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence` | String **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | [escape sequence](https://github.com/rtaylor034/axbind_2#escape-sequence) that is recognized by AxBind when reading the 'AxBind files' that the [Layer](https://github.com/rtaylor034/axbind_2#layer) affects. |
| `key_format` | [Key String](https://github.com/rtaylor034/axbind_2#key-string) **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Format that keys must be in to be recognized and mapped in each 'AxBind file'. the wildcard represents a raw key of the primary [Map](https://github.com/rtaylor034/axbind_2#map). |

#### Example:
```toml
#<...>
options.key_format = '%^%'
#<...>
```

# Known Issues

#### '@INCLUDE' keys are overriden based on alphabetical order:

#### Functions are always applied after remaps:

#### Functions are slow to evaluate:

#### Functions evaluate for every value of a map, regardless of use:

#### Layered escape sequence behavior may be unexpected (to the user):

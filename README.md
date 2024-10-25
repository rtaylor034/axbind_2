
# AxBind

> Dotfile management for the indecisive.
> 
[![Gfunction Status](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Frtaylor034%2Fproject-tags%2Fmain%2Ftags%2Fstatus%2Fusable.json)](https://github.com/rtaylor034/project-tags)
[![Gfunction Stance](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Frtaylor034%2Fproject-tags%2Fmain%2Ftags%2Fstance%2Finactive.json)](https://github.com/rtaylor034/project-tags)

## Description

AxBind is program intended to automate the task of editing multiple dotfiles when making a single abstract change to keybindings/colors/etc.

This is achieved through user-defined mappings that are applied to all/specified text files of a directory (ex: ~/.config).

Beyond basic key-value replacement, mapping behavior can be further controlled per-file (i.e. remapping, text functions, layering multiple maps, etc.).

AxBind is written in Rust and all configuration files of AxBind use [toml](https://toml.io/en/) syntax.

# Command Usage

#### ```axbind <root directory> [<options>]```

`<root directory>`: \
Directory to recursively search for tagged directories and apply AxBind mappings (ex: `~/.config`).

### Options

`--config=<path>`: \
Sets the [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file) path of this AxBind execution to `<path>`. 

*If unset, will check these paths in-order:*
1. `$XDG_CONFIG_HOME/axbind/config.toml`
2. `$HOME/.config/axbind/config.toml`
3. `/etc/axbind/config.toml`


# Documentation

**For quickly getting started, an [example configuraton](https://github.com/rtaylor034/axbind_2/tree/main/example_config) is provided with reasonable defaults and guiding comments.**

### About The Docs


All configuration files are [toml](https://toml.io/en/) tables, and will described as shown:

| Key | Type | Description |
|:----|:-----|:------------|
| `example_key` | Example **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Description of what the value for this key represents. |

All keys are required unless it's 'Type' is marked with a **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** as shown above.

The documentation defines toml sub-objects in the same format.

## Master Config File

Defines how AxBind searches and reads the filesystem and interprets it's own sub-config files.

| Key | Type | Description |
|:----|:-----|:------------|
| `map_directory` | String | Path that AxBind reads for [Map Files](https://github.com/rtaylor034/axbind_2#map-file), relative to this Master Config File's directory.  |
| `function_directory` | String | Path that AxBind reads for [Function Files](https://github.com/rtaylor034/axbind_2#function-file), relative to this Master Config File's directory. |
| `tag_directory` | String | Path of directory that AxBind recursively searches for inside the specified [root directory](https://github.com/rtaylor034/axbind_2#root-directory). Matching paths are deemed 'tag directories', and the directory *containing* them are "tagged" for AxBind modification. |
| `tag_entry_point` | String | Path to the [Tag Entry Point File](https://github.com/rtaylor034/axbind_2#tag-entry-point-file) in each 'tag directory'. |
| `options.meta` | [Meta Options](https://github.com/rtaylor034/axbind_2#meta-options) | Default [Meta Options](https://github.com/rtaylor034/axbind_2#meta-options). *Currently the only place where [Meta Options](https://github.com/rtaylor034/axbind_2#meta-options) are specified.* |
| `options.group` | [Group Options](https://github.com/rtaylor034/axbind_2#group-options) | Default [Group Options](https://github.com/rtaylor034/axbind_2#group-options) if they are unspecified in a [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file). |
| `options.layer` | [Layer Options](https://github.com/rtaylor034/axbind_2#layer-options) | Default [Layer Options](https://github.com/rtaylor034/axbind_2#layer-options) if they are unspecified in a [Layer](https://github.com/rtaylor034/axbind_2#layer). |

### Example
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

Defines a [Map](https://github.com/rtaylor034/axbind_2#map). \
Contained within the map directory.

| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_map` | String | Name of the [Map](https://github.com/rtaylor034/axbind_2#map) represented. |
| `map` | [Map](https://github.com/rtaylor034/axbind_2#map) | [Map](https://github.com/rtaylor034/axbind_2#map) being represented. |

### Example
```toml
# ~/.config/axbind/maps/myMap.toml
axbind_map = 'myMap'
[map]
foo = 'myFooReplacement'
bar = 'myBarReplacement'
```

## Map

A user-specified mapping of key-value pairs.

Component of [Map File](https://github.com/rtaylor034/axbind_2#map-file).

| Key | Type | Description |
|:----|:-----|:------------|
| \<any> | String | Key-value pairs that make up this Map. |
| `@INCLUDE` | String[] **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | List of Map names; This Map will inherit all key-value pairs of the Maps specified, in-order (Maps specified last will override duplicate keys). Key-value pairs directly specified in this Map *(are supposed too [See [Issues](https://github.com/rtaylor034/axbind_2/issues)])* override included Maps. |

#### Example
```toml
#<...>
foo = 'myFooReplacement'
bar = 'myBarReplacement'
#<...>
```

## Function File

Defines a [Function](https://github.com/rtaylor034/axbind_2#function).

| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_function` | String | Nme of the [Function](https://github.com/rtaylor034/axbind_2#function) represented. |
| `function` | [Function](https://github.com/rtaylor034/axbind_2#function) | [Function](https://github.com/rtaylor034/axbind_2#function) being represented. |

### Example
```toml
# ~/.config/axbind/functions/myFunction.toml
axbind_function = 'myFunction'
[function]
shell = 'sh'
command = 'echo -n "This used to be ^"'
```

## Function

A user-specified string-to-string function. \
Can only be used to *remap* values.

Component of [Function File](https://github.com/rtaylor034/axbind_2#function-file).

| Key | Type | Description |
|:----|:-----|:------------|
| `shell` | String | Shell executable/command that runs the body command. *The function is executed as `<shell> -c "<command>"`*. |
| `command` | [Key String](https://github.com/rtaylor034/axbind_2#key-string) | Shell command; Function body. The wildcard represents an unmapped value (input), and the standard out is the output of this Function. |

### Example
```toml
#<...>
shell = 'sh'
command = 'echo -n "This used to be ^"'
#<...>
```

## Tag Entry Point File

Defines the order in which [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file) will be applied to the parent directory. \
A Tag Entry Point File must be present in every tag directory.

If only one group is desired, you may treat a Tag Entry Point File as a [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file).

| Key | Type | Description |
|:----|:-----|:------------|
| `groups`* | String[] **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | List of paths to [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file) relative to the 'tag directory'. [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file) are evaluated in the order specified. |

### Example

```toml
# ~/.config/lf/.axbind/main.toml
groups = [
  'colors.toml',
  'keybindings.toml'
]
```

## Tag Group File

Defines [Layers](https://github.com/rtaylor034/axbind_2#layer) and the files they should be applied too.

If multiple Tag Group Files specify the same file, the **last** one, according to the order in the [Tag Entry Point File](https://github.com/rtaylor034/axbind_2#tag-entry-point-file), will take **full** precedence.

| Key | Type | Description |
|:----|:-----|:------------|
| `files` | String[] | List of file paths relative to the \*tagged\* directory that this group affects. Specified file paths will be *written* to after reading each files respective 'AxBind file' (See [Group Options](https://github.com/rtaylor034/axbind_2#group-options)). |
| `options` | [Group Options](https://github.com/rtaylor034/axbind_2#group-options) **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | This group's options. Overrides the defaults specified in the [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file). |
| `layers` | [Layer](https://github.com/rtaylor034/axbind_2#layer)[] | [Layers](https://github.com/rtaylor034/axbind_2#layer) to apply—in order—to all axbind files that this group affects. [Layers](https://github.com/rtaylor034/axbind_2#layer) are applied one-after-another and will read the output of the previous. |

### Example
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

A specification on how AxBind should work on a file.

Component of [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file).

| Key | Type | Description |
|:----|:-----|:------------|
| `map` | String | [Map](https://github.com/rtaylor034/axbind_2#map) name; all instances of this [Map](https://github.com/rtaylor034/axbind_2#map)'s keys in the specified [key format](https://github.com/rtaylor034/axbind_2#layer-options) will be replaced with its respective value (after specified `remaps` and `functions` are applied to it) when this [Layer](https://github.com/rtaylor034/axbind_2#layer) is applied. |
| `remaps` | String[] | List of [Map](https://github.com/rtaylor034/axbind_2#map) names; each value of `map` will be re-mapped (values used as keys) by these [Maps](https://github.com/rtaylor034/axbind_2#map), one-after-another in-order. |
| `functions` | String[] | List of [Function](https://github.com/rtaylor034/axbind_2#function) names; each value of `map` will be modified by these [Functions](https://github.com/rtaylor034/axbind_2#function), one-after-another in-order. *`functions` are applied *after* all remaps (see [Issues](https://github.com/rtaylor034/axbind_2/issues)).* |
| `options` | [Layer Options](https://github.com/rtaylor034/axbind_2#layer-options)[] **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | This Layer's options. Overrides the defaults specified in the [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).  |

### Example
```toml
#<...>
map = 'myKeybindings'
remaps = [ 'myLfRemaps' ]
options.key_format = '%^%'
#<...>
```

## Meta Options

Options relating to how AxBind interprets its own config files.

Component of [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).

| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence` | String **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | [Escape sequence](https://github.com/rtaylor034/axbind_2#escape-sequence) that is recognized by AxBind when interpreting [Key Strings](https://github.com/rtaylor034/axbind_2#key-string). *Currently is only useful for escaping the `wildcard_char`.* |
| `wildcard_char` | String **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Must be a single character; character representing the arbitrary input/value in a [Key String](https://github.com/rtaylor034/axbind_2#key-string), replaced with an actual value at evaluation depending on the context. *(See [Key String](https://github.com/rtaylor034/axbind_2#key-string))* |

### Example
```toml
#<...>
escape_sequence = '|'
wildcard_char = '^'
#<...>
```

### Text Evaluation Definitions
**Key String:** \
A string that, when evaluated, has all instances of the 'wildcard' (`wildcard_char`) replaced with another value. \
*Wildcards can be [escaped](https://github.com/rtaylor034/axbind_2#escape-sequence).*

**Escape Sequence:** \
A character sequence that, when encountered during text-evaluation, is replaced with nothing and 'escapes' the very next character.

An escaped character is **absolutely** ignored by all text-evaluation rules, as if it is not there, but will be present in the final text. \
*This is regardless of if the escaped character is "special" or not.* 



### Text Evaluation Example
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

Options relating to [Tag Group Files](https://github.com/rtaylor034/axbind_2#tag-group-file).

Component of [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file), [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).

| Key | Type | Description |
|:----|:-----|:------------|
| `axbind_file_format` | [Key String](https://github.com/rtaylor034/axbind_2#key-string) **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Format of each file's respective 'AxBind file'. the wildcard represents a name of a file the [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file) applies too. *(See [Tag Group File](https://github.com/rtaylor034/axbind_2#tag-group-file).`files`)* |

### Example
```toml
#<...>
axbind_file_format = '^.myCustomExtension'
#<...>
```

## Layer Options

Options relating to [Layers](https://github.com/rtaylor034/axbind_2#layer).

Component of [Layer](https://github.com/rtaylor034/axbind_2#layer), [Master Config File](https://github.com/rtaylor034/axbind_2#master-config-file).

| Key | Type | Description |
|:----|:-----|:------------|
| `escape_sequence` | String **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | [escape sequence](https://github.com/rtaylor034/axbind_2#escape-sequence) that is recognized by AxBind when reading the 'AxBind files' that the [Layer](https://github.com/rtaylor034/axbind_2#layer) affects. |
| `key_format` | [Key String](https://github.com/rtaylor034/axbind_2#key-string) **[?](https://github.com/rtaylor034/axbind_2#about-the-docs)** | Format that keys must be in to be recognized and mapped in each 'AxBind file'. the wildcard represents a raw key of the primary [Map](https://github.com/rtaylor034/axbind_2#map). |

### Example
```toml
#<...>
options.key_format = '%^%'
#<...>
```

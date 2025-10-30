# Usage

There are two main ways to interact with Nomad:

- through the Neovim command line, using the `:Mad` command;
- through the Lua API, by requiring the `"nomad"` module;

For example, you can start a new collaborative editing session with either:

```vim
:Mad collab start
```

or

```lua
require("nomad").collab.start()
```

In general, any command following the `:Mad <module> <action>` pattern has a
corresponding Lua function under `require("nomad").<module>.<action>`. Use the
command line for quick, interactive operations, and the Lua API when writing
scripts or setting up keybindings.

For brevity, the rest of this document will only show commands in the `:Mad
<module> <action>` format.

## `:Mad auth login`

## `:Mad auth logout`

## `:Mad collab start`

## `:Mad collab join`

## `:Mad collab copy-id`

## `:Mad collab jump <github_handle>`

## `:Mad collab leave`

## `:Mad collab pause`

## `:Mad collab resume`

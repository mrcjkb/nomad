---@class (exact) nomad.neovim.build
---
--- Builders.
---@field builders nomad.neovim.build.builders
---
--- Build contexts.
---@field contexts nomad.neovim.build.contexts

---@type nomad.neovim.build
return {
  builders = require("nomad.neovim.build.builders"),
  contexts = require("nomad.neovim.build.contexts"),
}

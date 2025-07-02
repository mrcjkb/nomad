---@class (exact) nomad.neovim.build
---
---Contexts.
---@field contexts nomad.neovim.build.contexts
---
---Builders.
---@field builders nomad.neovim.build.builders

---@type nomad.neovim.build
return {
  builders = require("nomad.neovim.build.builders"),
  contexts = require("nomad.neovim.build.contexts"),
}

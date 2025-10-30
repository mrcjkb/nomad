---@class (exact) nomad.neovim.build.contexts
---
--- Drive the builder to completion within lazy.nvim's `build` function.
---@field lazy fun(): nomad.neovim.build.Context

---@type nomad.neovim.build.Context
local Context = require("nomad.neovim.build.context")

---@type nomad.neovim.build.contexts
return {
  lazy = function()
    return Context.new(require("nomad.neovim.build.contexts.lazy"))
  end
}

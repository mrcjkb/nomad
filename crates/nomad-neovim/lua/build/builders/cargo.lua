---@class (exact) nomad.neovim.build.CargoOpts

---@type nomad.result.ResultModule
local result = require("nomad.result")

---@param opts nomad.neovim.build.CargoOpts?
---@param ctx nomad.neovim.build.Context
return function(opts, ctx)
  vim.system({ "sleep", "4" }, {}, function(obj)
    ctx.emit(vim.inspect(obj))
    ctx.on_done(result.ok(nil))
  end)
end

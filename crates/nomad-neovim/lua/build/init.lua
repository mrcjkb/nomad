---@alias nomad.neovim.Build nomad.neovim.build.Build

---@class (exact) nomad.neovim.build.Build
---
---Something.
---@field build fun(builder: nomad.neovim.build.Builder)
---
---Something else.
---@field builders nomad.neovim.build.Builders

---@class (exact) nomad.neovim.build.Context
---
---@field emit fun(msg: string)
---@field on_done fun(res: nomad.Result<nil, string>)
---@field override fun(self: nomad.neovim.build.Context, overrides: table<string, any>): nomad.neovim.build.Context

local context = {}
context.__index = context

---@return nomad.neovim.build.Context
context.new = function()
  local self = {
    emit = function(msg)
      print(msg)
    end,
    on_done = function(res)
      if res:is_err() then
        print(res:unwrap_err())
      end
    end,
  }
  return setmetatable(self, context)
end

function context:override(overrides)
  return setmetatable(vim.tbl_extend("force", self, overrides), context)
end

---@type nomad.neovim.build.Build
return {
  build = function(builder) builder.build(context.new()) end,
  builders = require("nomad.neovim.build.builders"),
}

---@class (exact) nomad.neovim.build.Builder
---
--- Build with the given driver.
---@field build fun(self: nomad.neovim.build.Builder, build_ctx: nomad.neovim.build.Context)
---
--- Fallback.
---@field fallback fun(self: nomad.neovim.build.Builder, fallback_builder: nomad.neovim.build.Builder): nomad.neovim.build.Builder

---@alias nomad.neovim.build.BuildFn fun(ctx: nomad.neovim.build.Context): nomad.future.Future<nomad.Result<nil, string>>

local future = require("nomad.future")

---@type nomad.Result
local Result = require("nomad.result")

---@type nomad.neovim.build.Context
local Context = require("nomad.neovim.build.context")

local Builder = {}
Builder.__index = Builder

---@param build_fn nomad.neovim.build.BuildFn
---@return nomad.neovim.build.Builder
Builder.new = function(build_fn)
  local self = setmetatable({}, Builder)
  self.build_fn = build_fn
  return setmetatable(self, Builder)
end

---@param self nomad.neovim.build.Builder
---@param build_ctx nomad.neovim.build.Context
function Builder:build(build_ctx)
  build_ctx.block_on_build(self.build_fn(build_ctx), 3)
end

---@param self nomad.neovim.build.Builder
---@param fallback_builder nomad.neovim.build.Builder
---@return nomad.neovim.build.Builder
function Builder:fallback(fallback_builder)
  return Builder.new(function(build_ctx)
    return future.async(function(ctx)
      local build_res = self.build_fn(build_ctx):await(ctx)
      if build_res:is_err() then
        build_ctx.notify(build_res:unwrap_err())
        return fallback_builder.build_fn(build_ctx):await(ctx)
      else
        return Result.ok(nil)
      end
    end)
  end)
end

return Builder

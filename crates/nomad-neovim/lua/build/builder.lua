---@class (exact) nomad.neovim.build.Builder
---
---Build with the given driver.
---@field build fun(self: nomad.neovim.build.Builder, driver: nomad.neovim.build.Driver)
---
---Fallback.
---@field fallback fun(self: nomad.neovim.build.Builder, fallback_builder: nomad.neovim.build.Builder): nomad.neovim.build.Builder

local Context = require("nomad.neovim.build.context")

local Builder = {}
Builder.__index = Builder

---@param build_fn fun(ctx: nomad.neovim.build.Context): nomad.future.Future<nomad.Result<nil, string>>
---@return nomad.neovim.build.Builder
Builder.new = function(build_fn)
  local self = {
    build_fn = build_fn,
  }
  return setmetatable(self, Builder)
end

---@param self nomad.neovim.build.Builder
---@param driver nomad.neovim.build.Driver
function Builder:build(driver)
  local build_ctx = Context.new({ emit = driver.emit })
  local build_fut = self.build_fn(build_ctx)
  driver.block_on_build(build_fut)
end

---@param self nomad.neovim.build.Builder
---@param fallback_builder nomad.neovim.build.Builder
---@return nomad.neovim.build.Builder
function Builder:fallback(fallback_builder)
  return Builder.new(function(ctx)
    return self.build_fn(ctx)
        :and_then(function(res)
          if res:is_err() then
            ctx.emit(res:unwrap_err())
            return fallback_builder.build_fn(ctx)
          end
        end)
  end)
end

return Builder

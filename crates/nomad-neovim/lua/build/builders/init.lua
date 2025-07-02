---@class (exact) nomad.neovim.build.builders
---
---Build with Cargo.
---@field cargo fun(opts: nomad.neovim.build.CargoOpts?): nomad.neovim.build.Builder
---
---Download prebuilt binary from GitHub releases.
---@field download_prebuilt fun(opts: nomad.neovim.build.DownloadPrebuiltOpts?): nomad.neovim.build.Builder
---
---Build with Nix.
---@field nix fun(opts: nomad.neovim.build.NixOpts?): nomad.neovim.build.Builder


---@class (exact) nomad.neovim.build.Builder
---
---Build with the given driver.
---@field build fun(self: nomad.neovim.build.Builder, driver: nomad.neovim.build.Driver)
---
---Fallback.
---@field fallback fun(self: nomad.neovim.build.Builder, fallback_builder: nomad.neovim.build.Builder): nomad.neovim.build.Builder

local builder_module = function(builder_filename)
  return "nomad.neovim.build.builders." .. builder_filename
end

local cargo = require(builder_module("cargo"))
local download_prebuilt = require(builder_module("download_prebuilt"))
local nix = require(builder_module("nix"))

local Builder = {}
Builder.__index = Builder

---@param build fun(ctx: nomad.neovim.build.Context)
---@return nomad.neovim.build.Builder
Builder.new = function(build)
  local self = {
    build = build,
  }
  return setmetatable(self, Builder)
end

---@param self nomad.neovim.build.Builder
---@param ctx nomad.neovim.build.Context
function Builder:build(ctx)
  ctx.drive(self)
end

---@param self nomad.neovim.build.Builder
---@param fallback_builder nomad.neovim.build.Builder
---@return nomad.neovim.build.Builder
function Builder:fallback(fallback_builder)
  return Builder.new(function(ctx)
    self.build(ctx:override({
      on_done = function(res)
        if res:is_err() then
          ctx.emit(res:unwrap_err())
          fallback_builder.build(ctx)
        end
      end
    }))
  end)
end

---@type nomad.neovim.build.builders
return {
  cargo = function(opts)
    return Builder.new(function(ctx) return cargo(opts, ctx) end)
  end,
  download_prebuilt = function(opts)
    return Builder.new(function(ctx) return download_prebuilt(opts, ctx) end)
  end,
  nix = function(opts)
    return Builder.new(function(ctx) return nix(opts, ctx) end)
  end
}

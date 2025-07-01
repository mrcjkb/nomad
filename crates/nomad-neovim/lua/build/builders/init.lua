---@class (exact) nomad.neovim.build.Builder
---
---Something.
---@field fallback fun(self: nomad.neovim.build.Builder, fallback_builder: nomad.neovim.build.Builder): nomad.neovim.build.Builder

---@class (exact) nomad.neovim.build.Builders
---
---Something.
---@field cargo fun(opts: nomad.neovim.build.CargoOpts?): nomad.neovim.build.Builder
---
---Something.
---@field download_prebuilt fun(opts: nomad.neovim.build.DownloadPrebuiltOpts?): nomad.neovim.build.Builder
---
---Something.
---@field nix fun(opts: nomad.neovim.build.NixOpts?): nomad.neovim.build.Builder

local builder_module = function(builder_filename)
  return "nomad.neovim.build.builders." .. builder_filename
end

local cargo = require(builder_module("cargo"))
local download_prebuilt = require(builder_module("download_prebuilt"))
local nix = require(builder_module("nix"))

local builder = {}
builder.__index = builder

---@param build fun(ctx: nomad.neovim.build.Context)
---@return nomad.neovim.build.Builder
builder.new = function(build)
  local self = {
    build = build,
  }
  return setmetatable(self, builder)
end

---@param self nomad.neovim.build.Builder
---@return nomad.neovim.build.Builder
function builder:fallback(fallback_builder)
  return builder.new(function(ctx)
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

---@type nomad.neovim.build.Builders
return {
  cargo = function(opts)
    return builder.new(function(ctx) return cargo(opts, ctx) end)
  end,
  download_prebuilt = function(opts)
    return builder.new(function(ctx) return download_prebuilt(opts, ctx) end)
  end,
  nix = function(opts)
    return builder.new(function(ctx) return nix(opts, ctx) end)
  end
}

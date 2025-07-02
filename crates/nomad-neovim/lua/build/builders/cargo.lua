---@class (exact) nomad.neovim.build.CargoOpts

---@type nomad.neovim.Command
local Command = require("nomad.neovim.command")

---@param opts nomad.neovim.build.CargoOpts
---@param ctx nomad.neovim.build.Context
return function(opts, ctx)
  Command.new("cargo")
      :args({ "xtask", "neovim", "build", "--release" })
      :arg(vim.version().prerelease and "--nightly" or nil)
      :current_dir(ctx:repo_dir())
      :on_stdout(ctx.emit)
      :on_stderr(ctx.emit)
      :on_done(function(res)
        ctx.on_done(res:map_err(tostring))
      end)
end

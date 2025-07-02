---@class (exact) nomad.neovim.build.NixOpts

---@type nomad.neovim.Command
local Command = require("nomad.neovim.command")

---@param opts nomad.neovim.build.NixOpts
---@param ctx nomad.neovim.build.Context
return function(opts, ctx)
  Command.new("nix")
      :arg("build")
      :arg(".#neovim" .. (vim.version().prerelease and "-nightly" or ""))
      :arg("--accept-flake-config")
      :current_dir(ctx:repo_dir())
      :on_stdout(ctx.emit)
      :on_stderr(ctx.emit)
      :on_done(function(res)
        if res:is_err() then return ctx.on_done(res:map_err(tostring)) end

        return Command.new("cp")
            :args({ "result/lua/*", "lua/" })
            :current_dir(ctx:repo_dir())
      end)
      :on_done(function(res)
        ctx.on_done(res:map_err(tostring))
      end)
end

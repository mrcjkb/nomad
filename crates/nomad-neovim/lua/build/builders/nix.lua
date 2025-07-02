---@class (exact) nomad.neovim.build.NixOpts

---@type nomad.neovim.Command
local Command = require("nomad.neovim.command")

---@param opts nomad.neovim.build.NixOpts
---@param ctx nomad.neovim.build.Context
---@return nomad.future.Future<nomad.Result<nil, string>>
return function(opts, ctx)
  return Command.new("nix")
      :arg("build")
      :arg(".#neovim" .. (vim.version().prerelease and "-nightly" or ""))
      :arg("--accept-flake-config")
      :current_dir(ctx:repo_dir())
      :on_stdout(ctx.emit)
      :on_stderr(ctx.emit)
      :and_then(function(res)
        if res:is_err() then return res:map_err(tostring) end

        return Command.new("cp")
            :args({ "result/lua/*", "lua/" })
            :current_dir(ctx:repo_dir())
      end)
      :and_then(function(res) res:map_err(tostring) end)
end

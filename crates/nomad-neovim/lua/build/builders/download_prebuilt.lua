---@class (exact) nomad.neovim.build.DownloadPrebuiltOpts

---@type nomad.neovim.Command
local Command = require("nomad.neovim.command")

---@type nomad.Result
local Result = require("nomad.Result")

---@param nomad_version string
---@return nomad.Result<string, string>
local get_artifact_name = function(nomad_version)
  local arch = ({
    ["x64"] = "x86_64",
    ["arm64"] = "aarch64",
  })[jit.arch]

  if not arch then
    return Result.err(("unsupported architecture: %s"):format(jit.arch))
  end

  local os = ({
    ["linux"] = "linux",
    ["osx"] = "macos",
  })[jit.os:lower()]

  if not os then
    return Result.err(("unsupported OS: %s"):format(jit.os:lower()))
  end

  local version = vim.version()
  local neovim_version = ("%s.%s%s"):format(
    version.major, version.minor, version.prerelease and "-nightly" or ""
  )

  return Result.ok(("nomad-%s-for-neovim-%s-%s-%s.tar.gz")
    :format(nomad_version, neovim_version, os, arch))
end

local get_artifact_url = function(tag, artifact_name)
  return ("https://github.com/nomad/nomad/releases/download/%s/%s")
      :format(tag, artifact_name)
end

---@param opts nomad.neovim.build.DownloadPrebuiltOpts
---@param ctx nomad.neovim.build.Context
return function(opts, ctx)
  ---@type string
  local tag

  ---@type string
  local artifact_name

  ---@type string
  local out_dir

  Command.new("git")
      :args({ "describe", "--tags", "--exact-match" })
      :current_dir(ctx:repo_dir())
      :on_stdout(function(line) tag = line end)
      :on_done(function(res)
        -- We're not on a tag, so we can't download a pre-built artifact.
        if res:is_err() then return ctx.on_done(res) end
        if tag == nil then return ctx.on_done(Result.err("not on a tag")) end

        -- We don't offer pre-built artifacts for this machine.
        local nomad_version = tag:gsub("^v", "")
        local res = get_artifact_name(nomad_version)
        if res:is_err() then return ctx.on_done(res:map(function() end)) end

        artifact_name = res:unwrap()
        local url = get_artifact_url(tag, artifact_name)
        out_dir = ctx:repo_dir():join("result")

        -- Download the artifact from the releases page.
        return Command.new("curl")
            -- Follow redirects.
            :arg("--location")
            :arg("--output")
            :arg(out_dir:join(artifact_name))
            :arg(url)
            :on_stdout(ctx.emit)
      end)
      :on_done(function(res)
        if res:is_err() then return ctx.on_done(res:map_err(tostring)) end

        return Command.new("tar")
            :args({ "-xzf", out_dir:join(artifact_name) })
            :args({ "-C", out_dir })
      end)
      :on_done(function(res)
        if res:is_err() then return ctx.on_done(res:map_err(tostring)) end

        return Command.new("cp")
            :args({ out_dir:join("/lua/*"), "lua/" })
            :current_dir(ctx:repo_dir())
      end)
      :on_done(function(res)
        ctx.on_done(res:map_err(tostring))
      end)
end

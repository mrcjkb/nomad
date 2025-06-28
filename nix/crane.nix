{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      lib,
      rust,
      ...
    }:
    {
      _module.args.crane =
        let
          craneLib = ((inputs.crane.mkLib pkgs).overrideToolchain rust.mkToolchain).overrideScope (
            # Override Crane's 'filterCargoSources' to also keep all Lua files
            # and all symlinks under `lua/nomad`.
            final: prev: {
              filterCargoSources =
                path: type:
                (prev.filterCargoSources path type)
                || (lib.hasSuffix ".lua" (builtins.baseNameOf path))
                || (type == "symlink" && lib.hasInfix "lua/nomad" path);
            }
          );

          depsArgs = {
            inherit (rust) buildInputs nativeBuildInputs;
            # Crane will emit a warning if there's no `workspace.package.name`
            # set in the workspace's Cargo.lock, so add a `pname` here to
            # silence that.
            pname = "nomad";
            src = craneLib.cleanCargoSource (craneLib.path ../.);
            strictDeps = true;
          };
        in
        {
          lib = craneLib;

          commonArgs = depsArgs // {
            cargoArtifacts = craneLib.buildDepsOnly depsArgs;
            env = {
              # Crane will run all 'cargo' invocation with `--release` if this
              # is not unset.
              CARGO_PROFILE = "";
              # The .git directory is always removed from the flake's source
              # files, so set the latest commit's hash and timestamp via
              # environment variables or crates/version's build script will
              # fail.
              COMMIT_HASH = inputs.self.rev or (lib.removeSuffix "-dirty" inputs.self.dirtyRev);
              COMMIT_UNIX_TIMESTAMP = toString inputs.self.lastModified;
            };
          };
        };
    };
}

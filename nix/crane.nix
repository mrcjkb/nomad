{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      lib,
      common,
      rust,
      ...
    }:
    {
      _module.args.crane =
        let
          mkCraneLib =
            targetPkgs:
            ((inputs.crane.mkLib targetPkgs).overrideToolchain (
              p:
              let
                toolchain = rust.mkToolchain p;
              in
              # No fucking clue why this is necessary, but not having it causes
              # `lib.getExe' toolchain "cargo"` in the common.xtask derivation
              # to return a store path like
              # /nix/store/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee-rust-default-1.89.0-nightly-2025-06-22/bin/cargo
              toolchain.override {
                extensions = (toolchain.extensions or [ ]);
              }
            )).overrideScope
              (
                # Override Crane's 'filterCargoSources' to also keep all Lua
                # files and all symlinks under `lua/nomad`.
                final: prev: {
                  filterCargoSources =
                    path: type:
                    (prev.filterCargoSources path type)
                    || (lib.hasSuffix ".lua" (builtins.baseNameOf path))
                    || (type == "symlink" && lib.hasInfix "lua/nomad" path);
                }
              );

          mkCommonArgs =
            targetPkgs:
            let
              craneLib = mkCraneLib targetPkgs;

              depsArgs = {
                inherit (common) nativeBuildInputs;
                buildinputs = common.buildInputs;
                # buildinputs = common.buildInputs targetPkgs;
                pname = common.workspaceName;
                src = craneLib.cleanCargoSource (craneLib.path ../.);
                strictDeps = true;
              };
            in
            depsArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly depsArgs;
              env = {
                # Crane will run all 'cargo' invocations with `--release` if
                # this is not unset.
                CARGO_PROFILE = "";
                # The .git directory is always removed from the flake's source
                # files, so set the latest commit's hash and timestamp via
                # environment variables or crates/version's build script will
                # fail.
                COMMIT_HASH = inputs.self.rev or (lib.removeSuffix "-dirty" inputs.self.dirtyRev);
                COMMIT_UNIX_TIMESTAMP = toString inputs.self.lastModified;
              };
            };

          mkCrane = targetPkgs: {
            lib = mkCraneLib targetPkgs;
            commonArgs = mkCommonArgs targetPkgs;
            overridePkgs = targetPkgs2: mkCrane targetPkgs2;
          };
        in
        mkCrane pkgs;
    };
}

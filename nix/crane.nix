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
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
            pkgs:
            let
              toolchain = rust.mkToolchain pkgs;
            in
            toolchain.override {
              extensions = (toolchain.extensions or [ ]) ++ [
                # Needed by cargo-llvm-cov to generate coverage.
                "llvm-tools-preview"
              ];
            }
          );
        in
        {
          lib = craneLib;

          commonArgs =
            let
              args = {
                inherit (rust) buildInputs nativeBuildInputs;

                # Crane will emit a warning if there's no
                # `workspace.package.name` set in the workspace's Cargo.lock,
                # so add a `pname` here to silence that.
                pname = "nomad";

                src = craneLib.cleanCargoSource (craneLib.path ../.);

                strictDeps = true;

                env = {
                  # Crane will run all 'cargo' invocation with `--release` if
                  # this is not unset.
                  CARGO_PROFILE = "";
                  # The .git directory is always removed from the flake's
                  # source files, so set the latest commit's hash and timestamp
                  # via environment variables or crates/version's build script
                  # will fail.
                  COMMIT_HASH = inputs.self.rev or (lib.removeSuffix "-dirty" inputs.self.dirtyRev);
                  COMMIT_UNIX_TIMESTAMP = toString inputs.self.lastModified;
                };
              };
            in
            args // { cargoArtifacts = craneLib.buildDepsOnly args; };
        };
    };
}

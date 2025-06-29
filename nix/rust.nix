{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      lib,
      crane,
      ...
    }:
    {
      _module.args.rust = rec {
        mkToolchain =
          pkgs:
          (inputs.rust-overlay.lib.mkRustBin { } pkgs).fromRustupToolchainFile (
            crane.lib.path ../rust-toolchain.toml
          );

        toolchain = mkToolchain pkgs;
      };
    };
}

{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      lib,
      crane,
      rust,
      ...
    }:
    {
      _module.args.common = {
        # The list of libraries to be linked against needed to compile all the
        # crates in the workspace with only their default features enabled.
        buildInputs =
          with pkgs;
          lib.lists.optionals stdenv.isLinux [
            # Needed by crates/auth to let "keyring" access the Secret
            # Service.
            dbus
          ];

        # The list of executables that have to be in $PATH needed to compile
        # all the crates in the workspace with only their default features
        # enabled (excluding packages from the Rust toolchain like cargo and
        # rustc).
        nativeBuildInputs = with pkgs; [ pkg-config ];

        # A compiled version of the xtask executable defined in this workspace.
        xtask = crane.lib.buildPackage rec {
          inherit (crane.commonArgs) cargoArtifacts src strictDeps;
          pname = "xtask";
          cargoExtraArgs = "--bin xtask";
          doCheck = false;
          env = {
            # Crane will compile xtask in release mode if this is not unset.
            CARGO_PROFILE = "";
            WORKSPACE_ROOT = crane.commonArgs.src.outPath;
          };
          nativeBuildInputs = [
            # Needed to call `wrapProgram`.
            pkgs.makeWrapper
          ];
          # Needed to shell out to `cargo metadata`.
          postInstall = ''
            wrapProgram $out/bin/${pname} \
              --set CARGO ${lib.getExe' rust.toolchain "cargo"} \
              --set RUSTC ${lib.getExe' rust.toolchain "rustc"}
          '';
        };
      };
    };
}

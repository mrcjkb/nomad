{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    neovim-nightly-overlay = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-parts.follows = "flake-parts";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem =
        {
          inputs',
          pkgs,
          lib,
          ...
        }:
        let
          common = {
            devShells = {
              default = pkgs.mkShell { };
            };
            rust = {
              cargoLock = {
                lockFile = ./Cargo.lock;
                # TODO: remove after publishing private crates.
                outputHashes = {
                  "abs-path-0.1.0" = lib.fakeHash;
                  "cauchy-0.1.0" = lib.fakeHash;
                  "codecs-0.0.9" = lib.fakeHash;
                  "lazy-await-0.1.0" = lib.fakeHash;
                  "nvim-oxi-0.6.0" = lib.fakeHash;
                  "pando-0.1.0" = lib.fakeHash;
                  "puff-0.1.0" = lib.fakeHash;
                };
              };
              platform =
                let
                  nightly-toolchain = inputs'.fenix.packages.fromToolchainFile {
                    file = ./rust-toolchain.toml;
                    sha256 = "sha256-SISBvV1h7Ajhs8g0pNezC1/KGA0hnXnApQ/5//STUbs=";
                  };
                in
                pkgs.makeRustPlatform {
                  cargo = nightly-toolchain;
                  rustc = nightly-toolchain;
                };
            };
          };

          neovim =
            let
              buildPlugin =
                let
                  pname = "mad-neovim";
                  version = "0.1.0";
                in
                {
                  isNightly,
                  isRelease ? true,
                }:
                common.rust.platform.buildRustPackage {
                  inherit pname version;
                  inherit (common.rust) cargoLock;
                  src = ./.;
                  buildPhase =
                    let
                      nightlyFlag = lib.optionalString isNightly "--nightly";
                      releaseFlag = lib.optionalString isRelease "--release";
                    in
                    ''
                      runHook preBuild
                      cargo xtask build ${nightlyFlag} ${releaseFlag}
                      runHook postBuild
                    '';
                  installPhase = ''
                    runHook preInstall
                    mkdir -p $out
                    cp -r lua $out/
                    runHook postInstall
                  '';
                };
            in
            {
              packages = {
                zero-dot-eleven = buildPlugin { isNightly = false; };
                nightly = buildPlugin { isNightly = true; };
              };
              devShells = {
                zero-dot-eleven = pkgs.mkShell { };
                nightly = pkgs.mkShell { };
              };
            };
        in
        {
          packages = {
            neovim = neovim.packages.zero-dot-eleven;
            neovim-nightly = neovim.packages.nightly;
          };
          devShells = {
            default = common.devShells.default;
            neovim = neovim.devShells.zero-dot-eleven;
            neovim-nightly = neovim.devShells.nightly;
          };
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}

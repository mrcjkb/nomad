{
  ...
}:

{
  perSystem =
    {
      config,
      pkgs,
      lib,
      inputs',
      common,
      crane,
      rust,
      ...
    }:
    let
      xtask = "${common.xtask}/bin/xtask";

      crateInfos = builtins.fromJSON (
        builtins.readFile (
          pkgs.runCommand "crate-infos" { } ''
            ${xtask} neovim print-crate-infos > $out
          ''
        )
      );

      mkPackage =
        isNightly: if isNightly then inputs'.neovim-nightly-overlay.packages.default else pkgs.neovim;

      mkCIShell =
        {
          isNightly,
        }:
        {
          packages = [
            (rust.toolchain)
            (mkPackage isNightly)
          ];
        };

      mkDevShell =
        {
          isNightly,
        }:
        config.devShells.default.overrideAttrs (drv: {
          nativeBuildInputs = (drv.nativeBuildInputs or [ ]) ++ [
            (mkPackage isNightly)
          ];
        });

      mkPlugin =
        {
          isNightly,
          isRelease ? true,
          ...
        }:
        crane.lib.buildPackage (
          crane.commonArgs
          // {
            pname = crateInfos.name;
            version = crateInfos.version;
            doCheck = false;
            buildPhaseCargoCommand =
              let
                nightlyFlag = lib.optionalString isNightly "--nightly";
                releaseFlag = lib.optionalString isRelease "--release";
              in
              "${xtask} neovim build ${nightlyFlag} ${releaseFlag} --out-dir=$out";
            # Installation was already handled by the build command.
            doNotPostBuildInstallCargoBinaries = true;
            installPhaseCommand = "";
          }
        );

      mkTests =
        {
          isNightly,
        }:
        crane.lib.cargoTest (
          crane.commonArgs
          // {
            cargoTestExtraArgs = lib.concatStringsSep " " [
              "--package=tests"
              "--features=neovim${lib.optionalString isNightly "-nightly"}"
              "--no-fail-fast"
            ];
            nativeBuildInputs = (common.nativeBuildInputs or [ ]) ++ [
              (mkPackage isNightly)
            ];
          }
        );
    in
    {
      checks = {
        tests-neovim = mkTests { isNightly = false; };
        tests-neovim-nightly = mkTests { isNightly = true; };
      };
      ciDevShells = {
        tests-neovim = mkCIShell { isNightly = false; };
        tests-neovim-nightly = mkCIShell { isNightly = true; };
      };
      devShells = {
        neovim = mkDevShell { isNightly = false; };
        neovim-nightly = mkDevShell { isNightly = true; };
      };
      packages = {
        neovim = mkPlugin { isNightly = false; };
        neovim-nightly = mkPlugin { isNightly = true; };
        neovim-release-artifacts = pkgs.stdenv.mkDerivation {
          inherit (crateInfos) version;
          pname = "${crateInfos.name}-release-artifacts";
          src = null;
          dontUnpack = true;
          nativeBuildInputs = with pkgs; [
            gnutar
            gzip
          ];
          installPhase =
            let
              args = [
                {
                  inherit pkgs;
                  isNightly = false;
                }
                {
                  inherit pkgs;
                  isNightly = true;
                }
              ];

              mkArchiveName =
                args:
                let
                  inherit (common) workspaceName;
                  inherit (crateInfos) version;
                  neovimVersion = if args.isNightly then "nightly" else "stable";
                  arch = common.getArchString args.pkgs;
                  os = common.getOSString args.pkgs;
                in
                "${workspaceName}-${version}-for-neovim-${neovimVersion}-${os}-${arch}.tar.gz";

              archivePlugins =
                let
                  archivePlugin =
                    args:
                    let
                      archiveName = mkArchiveName args;
                      plugin = mkPlugin args;
                    in
                    "tar -czf \"$out/${archiveName}\" -C \"${plugin}\" lua";
                in
                builtins.map archivePlugin args;
            in
            ''
              runHook preInstall
              mkdir -p $out
              ${lib.concatStringsSep "\n" archivePlugins}
              runHook postInstall
            '';
        };
      };
    };
}

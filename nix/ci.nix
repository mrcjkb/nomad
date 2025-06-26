{
  lib,
  flake-parts-lib,
  ...
}:

{
  options = {
    perSystem = flake-parts-lib.mkPerSystemOption (
      {
        config,
        pkgs,
        crane,
        ...
      }:
      {
        options.ciDevShells = lib.mkOption {
          type =
            let
              inherit (lib) types;
            in
            types.attrsOf (
              types.submodule {
                options = {
                  buildInputs = lib.mkOption {
                    type = types.listOf types.package;
                    default = [ ];
                    description = "List of packages to add to buildInputs";
                  };
                  packages = lib.mkOption {
                    type = types.listOf types.package;
                    default = [ ];
                    description = "List of packages to add to packages";
                  };
                  env = lib.mkOption {
                    type = types.attrsOf types.str;
                    default = { };
                    description = "Environment variables to set";
                  };
                };
              }
            );
          default = { };
          description = "CI development shells configuration";
        };

        config = {
          devShells =
            let
              mkDevShell =
                devShell:
                let
                  cleanedDevShell = builtins.removeAttrs devShell [
                    "buildInputs"
                    "packages"
                    "env"
                  ];
                in
                pkgs.mkShell (
                  cleanedDevShell
                  // {
                    buildInputs = (crane.commonArgs.buildInputs or [ ]) ++ (devShell.buildInputs or [ ]);
                    packages = (crane.commonArgs.nativeBuildInputs or [ ]) ++ (devShell.packages or [ ]);
                    env = (devShell.env or { }) // {
                      CARGO_UNSTABLE_CHECKSUM_FRESHNESS = "true";
                    };
                  }
                );
            in
            lib.mapAttrs' (name: devShell: {
              name = "ci-${name}";
              value = mkDevShell devShell;
            }) config.ciDevShells;
        };
      }
    );
  };
}

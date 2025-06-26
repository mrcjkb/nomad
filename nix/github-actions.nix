{
  ...
}:

{
  perSystem =
    {
      inputs',
      crane,
      ...
    }:
    {
      apps.nix-develop-gha = {
        type = "app";
        program = "${inputs'.nix-develop-gha.packages.default}/bin/nix-develop-gha";
      };

      ciDevShells = {
        tests = {
          packages = with crane.lib; [
            cargo
            rustc
          ];
          env = {
            # Setting this will disable some tests that fail in headless
            # environments like CI.
            HEADLESS = "true";
          };
        };
      };
    };
}

{
  ...
}:

{
  perSystem =
    {
      pkgs,
      crane,
      ...
    }:
    {
      checks.tests = crane.lib.cargoTest (
        crane.commonArgs
        // {
          checkPhase = ''
            # Run unit tests.
            cargo test --workspace --no-fail-fast

            # Run integration tests.
            cargo test --package=tests
          '';
        }
      );

      ciDevShells.tests = {
        packages = [
          crane.lib.cargo
          crane.lib.rustc
          pkgs.cargo-nextest
        ];
      };
    };
}

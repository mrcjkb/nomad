{
  ...
}:

{
  perSystem =
    {
      crane,
      ...
    }:
    {
      packages.coverage = crane.lib.cargoLlvmCov (
        crane.commonArgs
        // {
          buildPhaseCargoCommand = ''
            # Run unit tests.
            (cd crates && cargo llvm-cov test --no-report)

            # Run integration tests.
            (cd tests && cargo llvm-cov test --no-report --features=auth,collab,mock,walkdir)

            # Generate coverage report.
            cargo llvm-cov report --codecov --output-path codecov.json
          '';
          installPhaseCommand = ''
            mkdir -p $out
            mv codecov.json $out/
          '';
          env = (crane.commonArgs.env or { }) // {
            # Setting this will disable some tests that fail in headless
            # environments like CI.
            HEADLESS = "true";
          };
        }
      );
    };
}

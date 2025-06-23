{
  ...
}:

{
  perSystem =
    {
      lib,
      crane,
      ...
    }:
    {
      checks.docs = crane.lib.cargoDoc (
        crane.commonArgs
        // {
          cargoDocExtraArgs = lib.concatStringsSep " " [
            "--all-features"
            "--no-deps"
            "--workspace"
          ];
          env = (crane.commonArgs.env or { }) // {
            RUSTFLAGS = "--deny warnings";
          };
        }
      );
    };
}

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
      checks.clippy = crane.lib.cargoClippy (
        crane.commonArgs
        // {
          cargoClippyExtraArgs = lib.concatStringsSep " " [
            "--all-features"
            "--all-targets"
            "--no-deps"
            "--workspace"
            "--"
            "--deny warnings"
          ];
        }
      );
    };
}

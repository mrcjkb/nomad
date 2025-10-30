# Configuration

Nomad doesn't currently expose any configuration options, so you can set it up
by passing an empty table to the `setup` function:

```lua
require("nomad").setup({})
```

or by using `opts = {}` if you're using lazy.nvim.

## Highlight Groups

The only customization available is configuring the highlight groups used to
display remote peers in collaborative sessions.

There are three classes of highlight groups:

- `NomadCollabPeerCursor{n}` - highlights a peer's cursor position;
- `NomadCollabPeerHandle{n}` - highlights a peer's GitHub handle
  (displayed above their cursor);
- `NomadCollabPeerSelection{n}` - highlights a peer's visual selection in a
  buffer;

The `{n}` suffix is an integer ranging from 1 to 8, allowing different peers to
be represented with different colors. The first peer to join a session uses
highlight groups with suffix `1`, the second peer uses `2`, and so on.

If more than 8 peers join a session, the highlights cycle back starting from
`1`, so that the `n`-th and `n+8`-th peers will always share the same highlight
groups.

By default, `NomadCollabPeerCursor{n}` links to `Cursor`,
`NomadCollabPeerHandle{n}` links to `PmenuSelf`, and
`NomadCollabPeerSelection{n}` links to `Visual`. This may or may not produce
aesthetically pleasing results, depending on your colorscheme. To improve this,
consider opening a PR to add support for Nomad's highlight groups to your
favorite colorscheme. Thanks!

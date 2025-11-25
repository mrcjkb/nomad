# Changelog

## [Unreleased]

## [2025.11.2] - 2025-11-13

- An error that could occur at startup if `vim.fn.stdpath('data')` returned a
  path with consecutive path separators (e.g. `/home/user//.local/share/nvim`)
  ([#30][pr-30]);

- A panic that could occur if the file associated with a watched buffer was
  modified outside of Neovim ([#25][pr-25]);

## [2025.11.1] - 2025-11-05

### Fixed

- Plugin installation failure on systems where `command` is not in `$PATH`
  ([#24][pr-24]);

## [2025.11.0] - 2025-11-04

Initial release.

[pr-30]: https://github.com/nomad/nomad/pull/30
[pr-25]: https://github.com/nomad/nomad/pull/25
[pr-24]: https://github.com/nomad/nomad/pull/24

[unreleased]: https://github.com/nomad/nomad/compare/2025.11.2...HEAD
[2025.11.2]: https://github.com/nomad/nomad/compare/2025.11.1...2025.11.2
[2025.11.1]: https://github.com/nomad/nomad/compare/2025.11.0...2025.11.1
[2025.11.0]: https://github.com/nomad/nomad/releases/tag/2025.11.0

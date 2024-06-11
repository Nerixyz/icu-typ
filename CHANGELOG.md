# Changelog

<!-- markdownlint-configure-file { "no-duplicate-heading": { "siblings_only": true } } -->

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
Types of changes:
    - Added for new features.
    - Changed for changes in existing functionality.
    - Deprecated for soon-to-be removed features.
    - Removed for now removed features.
    - Fixed for any bug fixes.
    - Security in case of vulnerabilities.
-->

## [Unreleased]

### Added

- The documentation is now hosted at [nerixyz.github.io/icu-typ](https://nerixyz.github.io/icu-typ/) and includes more examples.

## [0.1.1] - 2024-06-05

### Added

- `locale-info` - This function gets information about a locale identifier and how it was parsed.
- This changelog was added.

### Changed

- 🚧 `fmt-timezone`: `offset` is now a named (but required) parameter to be more in sync with `fmt-zoned-datetime` (see example).
- Updated README to show more locales.

### Fixed

- Removed redundant `bytes()` call

## [0.1.0] - 2024-05-31

### Added

- `fmt-date`
- `fmt-time`
- `fmt-datetime`
- `fmt-timezone` (🚧 experimental)
- `fmt-zoned-datetime` (🚧 experimental)

[unreleased]: https://github.com/Nerixyz/icu-typ/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Nerixyz/icu-typ/releases/tag/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Nerixyz/icu-typ/releases/tag/v0.1.0

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fixed `-h` and `--help` options not printing help message (cli.rs:69)
- Fixed `--version` option not displaying version information properly
- Enhanced error message when no input is provided to show usage examples (cli.rs:58-63)
- Removed unused import warning in url.rs

### Changed
- Help and version errors are now properly printed before exiting the application
- Error message for missing input now includes helpful usage examples

## [1.4.2] - Previous release
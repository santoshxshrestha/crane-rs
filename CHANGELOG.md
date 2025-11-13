# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Authentication feature to secure file uploads and downloads.
- Login page for authentication.

### Changed

- Refactored the way of handling the geting the metadata of the uploaded files in the download page.
- Changed the variable names in the types.rs file for better clarity.
- Refactored the error handling in the routes to provide more specific error messages.
- Refactored the project structure to improve maintainability and readability.

### Fixed

### Removed

---

## [0.1.0] - 2025-10-22

### Added

- Initial working state of the project.
- Core dependencies: `actix-web`, `clap`, `actix-multipart`, `actix-files`, `askama`, `webbrowser`, `local-ip-address`, `qr2term`, `walkdir`.
- Nix flake support for reproducible development environments.
- CI/CD pipeline using GitHub Actions.
- Configuration files for code formatting.
- MIT License.
- README and project documentation.

[Unreleased]: https://github.com/santoshxshrestha/crane-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/santoshxshrestha/crane-rs/releases/tag/v0.1.0

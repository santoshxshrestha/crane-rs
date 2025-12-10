# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
### Changed
### Fixed

## [0.1.4] - 2025-12-10

### Changed
- Updated dependencies to latest versions (actix-web, actix-files, and related ecosystem packages).

## [0.1.3] - 2025-11-26

### Added
- Support uploading multiple files in a single request.
- Added installation and uninstallation scripts for easier setup and removal of the application.
- Added instructions for using the installation script in the README and script/README.
- Added the status notification on the login page.

### Changed
- Refactored upload route to iterate and persist each incoming file.
- Updated `UploadForm` to hold `Vec<TempFile>`.
- Updated upload template to enable selecting multiple files and display list of selected filenames.
- Changed the functionality of the listeners to avoid displaying redirected page content in the status.
- Changed the UI of the upload page.
- Update docs and README for clarity and additional information about the installation.

### Fixed
- Correct htmx `afterSwap` handler to use `e.detail.xhr.responseURL`.
- Show notification banner after successful multi-file upload.
- Recreate temp directory after --nuke so uploads/download listing keep working.
- Show inline "No files available" message in download page template when list empty (replaces previous 404).
- Use dynamic temp directory path in file listing (removed hard-coded path).
- Fix responsive styling for upload page "Choose File" button.

## [0.1.1] - 2025-11-13

### Added

- Added dependency: `rpassword` for password hiding during creation.
- Authentication feature to secure file uploads and downloads.
- Login page for authentication.

### Changed

- Refactored the method of getting authentication key from args.
- Refactored the way of handling the getting the metadata of the uploaded files in the download page.
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

[Unreleased]: https://github.com/santoshxshrestha/crane-rs/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/santoshxshrestha/crane-rs/releases/tag/v0.1.4
[0.1.3]: https://github.com/santoshxshrestha/crane-rs/releases/tag/v0.1.3
[0.1.2]: https://github.com/santoshxshrestha/crane-rs/releases/tag/v0.1.2
[0.1.1]: https://github.com/santoshxshrestha/crane-rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/santoshxshrestha/crane-rs/releases/tag/v0.1.0

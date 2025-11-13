# crane-rs Scripts

## Installation

To install `crane-rs` on your system, you can use the provided installation script. This will:
- Ensure the Rust toolchain is installed
- Clone or update the `crane-rs` repository to `$HOME/crane-rs`
- Build the project in release mode
- Install the binary to `/usr/local/bin/crane-rs`

### Quick Install (via curl)
```sh
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/crane-rs/main/scripts/install.sh | bash
```

### Options
- `--dry-run` : Show what would be done without making changes
- `--repo=URL` : Use a custom repository URL
- `-h, --help` : Show help message

Example:
```sh
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/crane-rs/main/scripts/install.sh | bash -s -- --dry-run
```

## Uninstallation

To remove `crane-rs` from your system, use the uninstall script. This will:
- Remove the binary from `/usr/local/bin/crane-rs`
- Delete the `$HOME/crane-rs` repository directory

### Quick Uninstall (via curl)
```sh
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/crane-rs/main/scripts/uninstall.sh | bash
```

### Options
- `--dry-run` : Show what would be done without making changes
- `-h, --help` : Show help message

Example:
```sh
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/crane-rs/main/scripts/uninstall.sh | bash -s -- --dry-run
```

---

For more details, inspect the scripts directly or run with `--help` for usage information.

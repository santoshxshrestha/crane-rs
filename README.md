# crane-rs

A simple, self-contained file sharing server written in Rust using the Actix-web framework. Easily upload and download files from any device on your local network via a modern web interface.

## Features

- **Upload & Download Files:** Share files between devices using a clean, responsive web UI.
- **Easy Access:** Displays your local IP and a QR code for quick access from mobile devices.
- **Preload Files:** Optionally preload files to share at startup.
- **Nuke Option:** Easily delete all temporary shared files with a command-line flag.
- **Modern Rust Stack:** Built with Actix-web, Askama, and more.
- **Nix Support:** Run instantly with Nix, or build manually with Cargo.

## Quick Start

### Run with Nix (Recommended)

```sh
nix run github:santoshxshrestha/crane-rs
```

### Build & Run Manually

1. **Clone the repository:**
   ```sh
   git clone https://github.com/santoshxshrestha/crane-rs.git
   cd crane-rs
   ```
2. **Build with Cargo:**
   ```sh
   cargo build --release
   ```
3. **Run the server:**
   ```sh
   ./target/release/crane-rs
   ```

## Command-Line Options

| Option         | Description                              | Default |
| -------------- | ---------------------------------------- | ------- |
| `-p`, `--port` | Port to run the server on                | `8080`  |
| `-f`, `--file` | File(s) to preload and share             |         |
| `-n`, `--nuke` | Nuke (delete) all temporary shared files | `false` |

**Examples:**

- Run on port 9000:
  ```sh
  ./crane-rs --port 9000
  ```
- Preload files for sharing:
  ```sh
  ./crane-rs -f ./myfile.txt -f ./photo.jpg
  ```
- Nuke all shared files:
  ```sh
  ./crane-rs --nuke
  ```

## Web Interface

- **Home:** Links to upload and download pages.
- **Upload:** Select and upload files from your device.
- **Download:** Download any files currently shared on the server.

## Security & Limitations

- **Local Network Only:** Designed for trusted local networks. No authentication or encryption is provided.
- **Temporary Storage:** Uploaded and preloaded files are stored in your system's temp directory under `crane-rs`.
- **File Size Limits:** Supports uploads up to 10GB per file (configurable in code).

## License

MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions, issues, and feature requests are welcome! Please open an issue or PR on GitHub.

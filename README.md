# crane-rs

A simple, self-contained file sharing server written in Rust using the Actix-web framework. Easily upload and download files from any device on your local network via a modern web interface.

## Features

- **Upload & Download Files:** Share files between devices using a clean, responsive web UI.
- **Easy Access:** Displays your local IP and a QR code for quick access from mobile devices.
- **Preload Files:** Optionally preload files to share at startup.
- **Nuke Option:** Easily delete all temporary shared files with a command-line flag.
- **Authentication (Optional):** Protect your server with a password, requiring users to log in before accessing file sharing features.
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

### Enable Authentication (Optional)

To require users to log in with a password, start the server with the `--auth` flag:

```sh
./target/release/crane-rs --auth
```
You will be prompted to enter a password interactively. All users must log in with this password to access the server.

## Command-Line Options

| Option         | Description                              | Default |
| -------------- | ---------------------------------------- | ------- |
| `-p`, `--port` | Port to run the server on                | `8080`  |
| `-f`, `--file` | File(s) to preload and share             |         |
| `-n`, `--nuke` | Nuke (delete) all temporary shared files | `false` |
| `-a`, `--auth` | Enable password authentication           | `false` |

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
- Enable authentication:
  ```sh
  ./crane-rs --auth
  ```
  You will be prompted to enter a password interactively. All users must log in with this password to access the server.

## Web Interface

- **Home:** Links to upload and download pages.
- **Upload:** Select and upload files from your device.
- **Download:** Download any files currently shared on the server.
- **Login:** If authentication is enabled, users are redirected to the login page and must enter the password to access file sharing features.

## Security & Limitations

- **Local Network Only:** Designed for trusted local networks. Authentication is optional and can be enabled via the CLI.
- **Authentication (Optional):** Enable password protection for all web routes except login and authentication endpoints. When enabled, users must log in with a password to access file sharing features.
- **Temporary Storage:** Uploaded and preloaded files are stored in your system's temp directory under `crane-rs`.
- **File Size Limits:** Supports uploads up to 10GB per file (configurable in code).

## Authentication Details

- When authentication is enabled, all routes except `/login` and `/authentication` require a valid session cookie.
- Users are redirected to `/login` if not authenticated.
- The password is set interactively at server startup and is not stored anywhere.
- To disable authentication, simply omit the `--auth` flag when starting the server.

## License

MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions, issues, and feature requests are welcome! Please open an issue or PR on GitHub.

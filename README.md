# Rust FTP Client

This is a simple FTP client implemented in Rust. It supports basic FTP commands and can be used to connect to an FTP server, list files, retrieve files, and more.

## Available Commands

- `list`: List files in the current directory on the FTP server.
- `get`: Retrieve a file from the FTP server.
- `mget`: Retrieve multiple files from the FTP server.
- `ascii`: Set ASCII mode for file transfers.
- `binary`: Set binary mode for file transfers.
- `help`: Display a list of available commands.
- `quit`: Disconnect from the FTP server.

## Getting Started

1. Clone the repository:

```sh
git clone https://github.com/yourusername/rust_ftp.git
cd rust_ftp
```

2. Build the project:

```sh
cargo build
```

3. Run the project:

```sh
cargo run
```

4. When prompted, enter the IP address and port of the FTP server you want to connect to, as well as your username and password.

5. Once connected, you can enter commands to interact with the FTP server. Use the help command to see a list of available commands.

## Dependencies

This project has no external dependencies.

## License

This project is licensed under the MIT License - see the [License.md](LICENSE.md) file for details.

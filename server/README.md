# veryrezsi server

A backend for the expense calculator. It is a Rust project, which uses the Axum web-framework to create a REST API.

## Dependencies

- rustup 1.24.3 or later
- mysql 8 or later
- docker and docker-compose - for easier database setup or as the backend for frontend development
- optional cargo plugins for easier development (install with  `cargo install cargo-watch cargo-edit cargo-llvm-cov cargo-audit cargo-cache cargo-update`)
  - cargo-watch - hot-reload for development
  - cargo-edit - command line dependency utility
  - cargo-llvm-cov - test coverage tool
  - cargo-audit - vulnerability scanner
  - cargo-cache - dependency and build cache manager
  - cargo-update - updates the installed plugins (this is different from the `cargo update` command, which updates project dependencies)
- rust analyzer vscode plugin for development

## Environment

Variables can be set as EVs or in the `.env` file.
Used variables are:

- **HOST** - the host of the application
- **PORT** - the port the appliaction will listen to
- **DATABASE_URL**
  - the database the application will attempt to connect to
  - should be a valid mysql connection string
  - user and password should be provided
- **COOKIE_KEY**
  - cookies will be encrypted using this key
  - should be equal or greater than **64**
- **RUST_LOG** - log level, possible value are _info_, _debug_, _error_, _warn_, _trace_

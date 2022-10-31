# veryrezsi server

A backend for the expense calculator. It is a Rust project, which uses the Axum web-framework to create a REST API.

## Prerequisites

- _rustup_ - update with `rustup self update`
- _rustc_ - update with `rustup update`
- _mysql_
- _docker_ and _docker-compose_ - for easier database setup or as the backend for frontend development
- optional _cargo plugins_ for easier development (install with `cargo install cargo-watch cargo-llvm-cov cargo-audit cargo-cache cargo-update cargo-edit`)
  - _cargo-watch_ - hot-reload for development, use with `cargo watch -x run`
  - _cargo-audit_ - vulnerability scanner, use with `cargo audit`
  - _cargo-llvm-cov_ - test coverage tool, use with `cargo llvm-cov`
  - _cargo-cache_ - dependency and build cache manager
  - _cargo-update_ - updates installed binaries with `cargo install-update -a` (this is different from the `cargo update` command, which updates project dependencies)
  - _cargo-edit_ - `cargo upgrade --workspace --to-lockfile` automatically updates dependencies in Cargo.toml
- _sea-orm-cli_
  - install with `cargo install sea-orm-cli`
  - has many uses, but heavily optional because it is only used here for the generation of base migration files with `sea-orm-cli migrate generate migration_name`, which saves us some hassle
- _rust-analyzer_ vscode plugin for development

## Environment

Configuration options can be set as environment variables or in the `/resources/app-config.toml` file.
The environment variable names are:

- **SERVER_ADDRESS** - the host and port of the application
- **DATABASE_URL**
  - the database the application will attempt to connect to
  - should be a valid mysql connection string
  - user and password should be provided
- **COOKIE_KEY**
  - cookies will be encrypted using this key
  - should be equal or greater than **64**
- **LOG_LEVEL** - log level, possible value are _info_, _debug_, _error_, _warn_, _trace_
- **SMTP_ADDRESS** - the address of the smtp server
- **SMTP_PORT** - the port of the smtp server
- **SMTP_USERNAME** - the username for the smtp server
- **SMTP_PASSWORD** - the password for the smtp server

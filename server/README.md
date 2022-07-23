# veryrezsi server

A backend for the expense calculator. It is a Rust project, which uses the Axum web-framework to create a REST API.

## Dependencies

- _rustup_ (1.62.1 or later)
- _rustc_ (1.25.1 or later)
- _mysql_ (8 or later)
- _docker_ (20.10.17 or later) and _docker-compose_ (1.29.2 or later) - for easier database setup or as the backend for frontend development
- optional _cargo plugins_ for easier development (install with `cargo install cargo-watch cargo-llvm-cov cargo-audit cargo-cache cargo-update`)
  - _cargo-watch_ - hot-reload for development, use with `cargo watch -x run`
  - _cargo-audit_ - vulnerability scanner, use with `cargo audit`
  - _cargo-llvm-cov_ - test coverage tool
  - _cargo-cache_ - dependency and build cache manager
  - _cargo-update_ - updates installed binaries with `cargo install-update -a` (this is different from the `cargo update` command, which updates project dependencies)
- _sea-orm-cli_
  - install with `cargo install sea-orm-cli`
  - has many uses, but heavily optional because it is only used here for the generation of base migration files with `sea-orm-cli migrate generate migration_name`, which saves us some hassle
- _rust-analyzer_ vscode plugin for development

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
- **SMTP_ADDRESS** - the address of the smtp server
- **SMTP_PORT** - the port of the smtp server
- **SMTP_USERNAME** - the username for the smtp server
- **SMTP_PASSWORD** - the password for the smtp server

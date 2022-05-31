# veryrezsi server

A backend for the expense calculator. It a Rust project, which uses the Axum web-framework to create a REST API.

## Dependencies

**To develop and build locally**

- rustup 1.24.3 or later
- mysql 8 or later
- optional for easier development (install with `cargo install ${crate}`)
  - cargo-watch - for hot-reload (use with `cargo watch -x run`)
  - cargo-edit - command line dependency tool
  - cargo-llvm-cov - test coverage tool
  - cargo-audit - vulnerability scanner

**To run in Docker**

- only docker and docker-compose is needed

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

## Local setup

- Install rustup and setup a mysql database
- Run the `init.sql` file, which creates the necessary database structure
- Build and run with `cargo run`, it will automatically run database migrations
- To manipulate migrations, refer to the README.md in the `migration` directory and use the CLI tool
  - it uses the outer `.env` file for the connection string but it can be supplied with it's own file in the `migration` directory if needed

## Docker setup

setup and migration when needed

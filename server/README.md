# veryrezsi server

A backend for the expense calculator. It is a Rust project, which uses the Axum web-framework to create a REST API.

## Dependencies

- rustup 1.24.3 or later
- mysql 8 or later
- optional for easier development (install with `cargo install ${crate}`)
  - cargo-watch - for hot-reload (use with `cargo watch -x run`)
  - cargo-edit - command line dependency tool
  - cargo-llvm-cov - test coverage tool
  - cargo-audit - vulnerability scanner
  - docker and docker-compose - for easier database setup or as the backend for frontend development

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

## Backend dev setup

- Install rustup
- Setup database, there are two options
  - local mysql database
    - Run the `init.sql` file, which creates the necessary database structure
  - mysql database in a docker container with docker-compose
    - Run `docker-compose up -d database` and a database will be available at the 3306 port, already initialized
- Build and run with `cargo run`, it will automatically run database migrations
  - When developing use `cargo watch -x run` for hot-reloading, provided `cargo-watch` is installed
- To manipulate migrations, refer to the README.md in the `migration` directory and use the CLI tool
  - It uses the outer `.env` file for the connection string but it can be supplied with it's own file in the `migration` directory if needed

# Frontend dev setup

- To run the complete backend (database & server), use `docker-compose up -d`, the database will be available at port 3306 and the server on 8000
- The build will take some time at first and on changes, consequent runs will be much faster thanks to docker caching
- It can be used as a backend for frontend development, the server is built in release mode, so it is smaller and highly optimized
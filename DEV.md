# Backend dev setup

## Prerequisites

- _rustup_ 1.25.1 or higher
- _rustc_ 1.65.0 or higher
- _mysql_ 8 or higher
- _SMTP_ relay, of any kind 
### OR
- _docker_ 20.10.17 or higher
- _docker-compose_ 1.29.2 or higher

IDE
 - _Visual Studio Code_ is recommended.
   - with rust-analyzer (rust-lang.rust-analyzer) extension to work with rust
   - with CodeLLDB (vadimcn.vscode-lldb) extension to debug

## Starting a dev database

- There are two options:
  - local mysql database
    - Run the `database/init.sql` file, which creates the necessary database structure
  - mysql database in a docker container with docker-compose
    - Run `docker-compose up -d database` and a database will be available at the 3306 port, already initialized

## Starting mailhog

- Mailhog may be needed for some functionalities (registration), although if not present it only causes error messages in the log.
- Run it locally or by adding `mailhog` to the end of your `docker-compose up -d` command.

## Running server

- Build and run with `cargo run`, it will automatically run database migrations
  - When developing use `cargo watch -x run` for hot-reloading, provided `cargo-watch` is installed
  - Do not forget that you will need a database up and running to properly run the application. The easiest way is to run `docker-compose up -d database` in the root of the project
- To manipulate migrations, refer to the README.md in the `server/migration` directory and use the CLI tool
  - It uses the outer `.env` file for the connection string but it can be supplied with it's own file in the `server/migration` directory if needed

## Use the API
- There is a Postman collection file, ready to be used for testing during development at `server/postman_collections.json`
  - Don't just use it, remember to update it when any of the API schema changes. 

# Frontend dev setup

## Prerequisites

- Docker
- node 16.7 or higher
  - v16.14.0 ✅
  - v18.4.0 ✅
- npm v7 or higher
  - 8.4.1 ✅

## Starting backend for client

- To run the complete backend (database & server), use `docker-compose up -d server`, the database will be available at port 3306 and the server on 8000
- The build will take some time at first and on changes, consequent runs will be much faster thanks to docker caching
- It can be used as a backend for frontend development, the server is built in release mode, so it is smaller and highly optimized

## Starting Svelte dev server

- `cd ./client`
- `npm i`
- `npm run dev`

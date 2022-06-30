## Prerequisites 

- npm v7 or higher
  - 8.4.1 ✅
- node 16.7 or higher
  - v16.14.0 ✅

## Backend dev setup

- Install rustup
- Setup database, there are two options
  - local mysql database
    - Run the `server/init.sql` file, which creates the necessary database structure
  - mysql database in a docker container with docker-compose
    - Run `docker-compose up -d database` and a database will be available at the 3306 port, already initialized
- Build and run with `cargo run`, it will automatically run database migrations
  - When developing use `cargo watch -x run` for hot-reloading, provided `cargo-watch` is installed
- To manipulate migrations, refer to the README.md in the `migration` directory and use the CLI tool
  - It uses the outer `.env` file for the connection string but it can be supplied with it's own file in the `migration` directory if needed

# Frontend dev setup

## Starting backend for client
- To run the complete backend (database & server), use `docker-compose up -d database server`, the database will be available at port 3306 and the server on 8000
- The build will take some time at first and on changes, consequent runs will be much faster thanks to docker caching
- It can be used as a backend for frontend development, the server is built in release mode, so it is smaller and highly optimized

## Starting Svelte dev server
- `cd ./client`
- `npm i`
- `npm run dev`
FROM lukemathwalker/cargo-chef:latest-rust-1-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin veryrezsi

FROM debian:bullseye AS runtime
WORKDIR /app
COPY ./docker/config/app-config.toml /app/resources/app-config.toml
COPY --from=builder /app/target/release/veryrezsi /usr/local/bin
ENTRYPOINT ["/usr/local/bin/veryrezsi"]

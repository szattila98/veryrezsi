FROM rust:latest as builder

RUN USER=root cargo new --bin veryrezsi
WORKDIR /veryrezsi

COPY ./Cargo.toml ./Cargo.toml
COPY ./entity ./entity
COPY ./migration ./migration
COPY ./src ./src
COPY ./resources ./resources

RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /veryrezsi/target/release/veryrezsi ${APP}/veryrezsi
COPY ./docker/config/.env ${APP}/.env

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./veryrezsi"]
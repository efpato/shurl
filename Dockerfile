FROM rust:slim as build

RUN apt-get update && \
    apt-get install --no-install-recommends -y libpq-dev && \
    rm -rf /var/lib/apt/lists/* && \
    USER=root cargo new --bin app

WORKDIR /app

COPY Cargo.toml Cargo.toml

RUN cargo build && \
    rm src/*.rs

COPY ./migrations ./migrations
COPY ./src ./src
COPY ./diesel.toml .

RUN rm ./target/debug/deps/shurl* && \
    cargo build --release


FROM debian:buster-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install --no-install-recommends -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/shurl .
COPY ./configs ./configs

CMD ["./shurl"]

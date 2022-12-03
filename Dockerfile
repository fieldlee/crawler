FROM rust:1.64.0 as builder
# Use prebuilt builder image
# FROM rust:1.50-prebuilt as builder
WORKDIR /usr/src

ARG APP=crawler

# New cargo project and copy Rust dependencies (and store as a separate Docker layer)
# NOTE: must call `cargo vendor` first and add `vendor` folder to git
RUN USER=root cargo new crawler

WORKDIR /usr/src/crawler

RUN mkdir -p .cargo

COPY .cargo/config.toml .cargo/

COPY vendor vendor

COPY yt-api yt-api

COPY Cargo.toml application.yaml Cargo.lock ./


COPY ./src src

RUN cargo build --release --frozen --bin crawler

ENTRYPOINT ["/usr/src/crawler/crawler"]
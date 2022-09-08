FROM rust:1.63-alpine as builder

# COPY ./ ./

# RUN cargo build --release

# CMD ["./target/release/rss2email"]

RUN apk add --no-cache musl-dev
WORKDIR /opt
RUN cargo new --bin rss2email
WORKDIR /opt/rss2email
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm ./src/*.rs
RUN rm ./target/release/deps/rss2email*

ADD ./src ./src
RUN cargo build --release

FROM scratch
WORKDIR /opt/rss2email
COPY --from=builder /opt/rss2email/target/release/rss2email .
COPY ./.env ./.env
COPY ./feeds.txt ./feeds.txt

CMD ["./rss2email"]
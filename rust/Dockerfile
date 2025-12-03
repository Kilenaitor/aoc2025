FROM rust:alpine

RUN apk add --no-cache musl-dev
RUN cargo install cargo-watch

COPY . /app/aoc2025
WORKDIR /app/aoc2025

CMD ["cargo", "watch", "-x", "run"]

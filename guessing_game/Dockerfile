FROM rust:latest as cargo-build

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/guessing_game

COPY Cargo.toml Cargo.toml
RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/guessing_game*

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
RUN addgroup -g 1000 guessing_game
RUN adduser -D -s /bin/sh -u 1000 -G guessing_game guessing_game
WORKDIR /home/guessing_game/bin/
COPY --from=cargo-build /usr/src/guessing_game/target/x86_64-unknown-linux-musl/release/guessing_game .
RUN chown guessing_game:guessing_game guessing_game
USER guessing_game
CMD ["./guessing_game"]

FROM rust:1.61.0 as builder

RUN rustup target add x86_64-unknown-linux-musl 

WORKDIR /app
RUN cargo new karsher
WORKDIR /app/karsher

COPY ./Cargo.toml ./Cargo.lock ./

ENV RUSTFLAGS='-C link-arg=-s'

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf ./src

COPY ./src/ ./src

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/karsher*

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine

ENV RUST_LOG=info

VOLUME /root/.local/share

COPY --from=builder  /app/karsher/target/x86_64-unknown-linux-musl/release/karsher .
CMD [ "/karsher" ]
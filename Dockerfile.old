FROM rust:1.75-bookworm as builder

WORKDIR /app

RUN cargo new adana

WORKDIR /app/adana

COPY rust-toolchain.toml .

COPY ./Cargo.toml ./Cargo.lock ./

COPY .cargo/config .cargo/config

# ENV RUSTFLAGS='-C link-arg=-s'

RUN cargo build --release 

RUN rm -rf ./src

COPY ./src/ ./src

RUN rm ./target/release/deps/adana*

RUN cargo build --release 

FROM rust:1.73-slim-bookworm

ENV RUST_LOG=info

VOLUME /root/.local/share

COPY --from=builder  /app/adana/target/release/adana .

ENTRYPOINT [ "/adana" ]

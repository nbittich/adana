FROM rust:1.80-bookworm as builder

WORKDIR /app

RUN cargo new adana

WORKDIR /app/adana

COPY . .

RUN cargo build --release 

FROM rust:1.80-slim-bookworm

ENV RUST_LOG=info

VOLUME /root/.local/share

COPY --from=builder  /app/adana/target/release/adana .

ENTRYPOINT [ "/adana" ]

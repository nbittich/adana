#!/usr/bin/env bash
set -e 

if [[ -z "$1" ]]
then
  echo "version mut be provided"
  exit 1;
fi

RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/karsher dist/musl/karsher-$1
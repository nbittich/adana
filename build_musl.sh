#!/usr/bin/env bash
set -e 

if [[ -z "$1" ]]
then
  echo "version mut be provided"
  exit 1;
fi

mkdir -p static_binaries
cargo build --release 
cp target/x86_64-unknown-linux-musl/release/adana static_binaries/adana-$1

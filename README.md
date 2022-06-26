# Karsher -  dumb cache / dumb terminal

- use help for help
- use static build under dist/musl or use cargo build / cargo run


Static build using MUSL:

```
rustup target add x86_64-unknown-linux-musl 
```


```
RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl
```

should print "statically linked":

```
ldd target/x86_64-unknown-linux-musl/release/karsher 

```

### install using cargo
RUSTFLAGS='-C link-arg=-s' cargo install --target x86_64-unknown-linux-musl karsher

### logs
RUST_LOG=debug karsher
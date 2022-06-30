# Karsher -  dumb cache / dumb terminal

- use help for help
- use static build under dist/musl or use cargo build / cargo run


Static build using MUSL:

```
cargo build --release 
```

should print "statically linked":

```
ldd target/x86_64-unknown-linux-musl/release/karsher 

```

### install using cargo
cargo build --release 

### logs
RUST_LOG=info karsher


### docker

```
docker build -t karsher .

docker run -it karsher 

```
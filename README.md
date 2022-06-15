# Karsher -  dumb cache / dumb terminal

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

### todos
- ~~rustyline (https://crates.io/crates/rustyline)~~



https://user-images.githubusercontent.com/3816305/173184980-d5152f9e-acf8-4f45-b97f-aeea472d52ac.mp4


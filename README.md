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
- add / store / load env variables
- autosave: when two shells are open, which one to save? 



https://user-images.githubusercontent.com/3816305/173921007-761b1a22-00b4-4fe3-a657-4aee0dce344c.mp4


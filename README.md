# Karsher -  dumb cache / dumb terminal

- use help for help
- use static build under dist/musl or use cargo build / cargo run
- you can populate the db the first time by running restore (you must have the karsherdb.json from this repo saved on your current directory)


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
- ~~autosave: when two shells are open, which one to save?~~
- add / store / load env variables
- add time like in nu



https://user-images.githubusercontent.com/3816305/173921007-761b1a22-00b4-4fe3-a657-4aee0dce344c.mp4


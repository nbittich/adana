# Karsher -  dumb cache / dumb terminal

- use help for help
- use static build under dist/musl or use cargo build / cargo run

``` 
╭───┬────────┬─────────┬──────────┬──────┬─────────┬─────────╮
│ # │  pid   │  name   │  status  │ cpu  │   mem   │ virtual │
├───┼────────┼─────────┼──────────┼──────┼─────────┼─────────┤
│ 0 │ 886916 │ karsher │ Sleeping │ 0.00 │ 4.0 KiB │ 5.3 MiB │
╰───┴────────┴─────────┴──────────┴──────┴─────────┴─────────╯

-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            15            325             16           2110
-------------------------------------------------------------------------------
SUM:                            15            325             16           2110

``` 

Static build using MUSL:

```
cargo build --release 
```

should print "statically linked":

```
ldd target/x86_64-unknown-linux-musl/release/karsher 

```

### install using cargo

```
cargo install karsher --target x86_64-unknown-linux-musl 
```

### logs
RUST_LOG=info karsher


### docker

```
docker build -t karsher .

docker run -it karsher 

```

### Args - override

``` 
# open an in memory db

karsher --inmemory

```

```
# override db path & history path + fallback in memory in case of an error (default to false)
# path must exist! file doesn't have to.

karsher --dbpath /tmp/mydb.db --historypath /tmp/myhistory.txt --fallback

```
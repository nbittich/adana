# Karsher -  dumb cache 

```
nbittich@pop-os:~/toy-program/karsher$ cargo run
   Compiling karsher v0.1.0 (/home/nbittich/toy-program/karsher)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/karsher`
> add -a drc docker-compose
added docker-compose with hash key 15609331997961958896 and aliases [(9353934320943266248, "drc")]
> get 15609331997961958896
found docker-compose
> get drc
found docker-compose
> del drc
removed docker-compose with hash key 15609331997961958896
> get drc
drc not found
> 
```
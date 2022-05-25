# Karsher -  dumb cache / dumb terminal

Static build using MUSL:

```
rustup target add x86_64-unknown-linux-musl 
```

```
RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl
```

should be empty:

```
readelf -d target/x86_64-unknown-linux-musl/release/karsher | grep NEEDED 

```

### todos
- ~~rustyline (https://crates.io/crates/rustyline)~~
- make it less dumb


```
>> help
> add : Add a new value to current cache. can have multiple aliases with option '-a'. e.g `add -a drc -a drcomp docker-compose`
> list/ls : List values within the cache.
> listcache/lsch : List available caches.
> del/delete : Remove value from cache. Accept either a hashkey or an alias. e.g `del drc`
> get : Get value from cache. Accept either a hashkey or an alias. e.g `get drc`
> exec/run : Run a value from the cache as an OS command. Accept either a hashkey or an alias. e.g `run drc`
> use/using : Use another cache context default cache is DEFAULT. e.g `use linux`
> dump : Dump cache(s) as json. Take an optional parameter, the cache name. e.g `dump linux`
> clear/cls : Clear the terminal.
> help : Display Help.
>> 
>> using linux
current cache: linux
>> add -a drc docker-compose
added docker-compose with hash key 15609331997961958896
>> add -a sshprod ssh root@localhost_prot
added ssh root@localhost_prot with hash key 9809177213078877385
>> add -a nux ls -alh
added ls -alh with hash key 15037829439261551135
>> using winapi
current cache: winapi
>> del nux
key nux not found in current cache winapi
>> add -a cls CLEAR
added CLEAR with hash key 14679148844497871129
>> add -a ls DIR
added DIR with hash key 969001501853342793
>> del ls 
removed DIR with hash key ls
>> dump winapi
{
  "cache": {
    "14679148844497871129": "CLEAR"
  },
  "cache_aliases": {
    "5189941954874582573": 14679148844497871129
  }
}
>> using linux
current cache: linux
>> dump linux
{
  "cache": {
    "9809177213078877385": "ssh root@localhost_prot",
    "15037829439261551135": "ls -alh",
    "15609331997961958896": "docker-compose"
  },
  "cache_aliases": {
    "9353934320943266248": 15609331997961958896,
    "12515839607359842530": 9809177213078877385,
    "15931045025146721018": 15037829439261551135
  }
}
>> del ls
key ls not found in current cache linux
>> del nux
removed ls -alh with hash key nux
>> dump
{
  "caches": {
    "linux": {
      "cache": {
        "9809177213078877385": "ssh root@localhost_prot",
        "15609331997961958896": "docker-compose"
      },
      "cache_aliases": {
        "9353934320943266248": 15609331997961958896,
        "12515839607359842530": 9809177213078877385
      }
    },
    "winapi": {
      "cache": {
        "14679148844497871129": "CLEAR"
      },
      "cache_aliases": {
        "5189941954874582573": 14679148844497871129
      }
    }
  }
}
>>  
```
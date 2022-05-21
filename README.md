# Karsher -  dumb cache 

```
> using linux
current cache: linux
> add -a drc docker-compose
added docker-compose with hash key 15609331997961958896
> add -a sshprod ssh root@localhost_prot
added ssh root@localhost_prot with hash key 9809177213078877385
> add -a nux ls -alh
added ls -alh with hash key 15037829439261551135
> using winapi
current cache: winapi
> del nux
key nux not found in current cache winapi
> add -a cls CLEAR
added CLEAR with hash key 14679148844497871129
> add -a ls DIR
added DIR with hash key 969001501853342793
> del ls 
removed DIR with hash key ls
> dump winapi
{
  "cache": {
    "14679148844497871129": "CLEAR"
  },
  "cache_aliases": {
    "5189941954874582573": 14679148844497871129
  }
}
> using linux
current cache: linux
> dump linux
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
> del ls
key ls not found in current cache linux
> del nux
removed ls -alh with hash key nux
> dump
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
>  
```
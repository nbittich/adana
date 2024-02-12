# Adana

Scripting programming language, repl and namespaced aliases for commands.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Programming language](#programming-language)
   - [Getting started](#getting-started)
   - [Comments](#comments)
   - [Multiline](#multiline)
   - [F-Strings](#f-strings)
   - [Operators and constants](#operators-and-constants)
   - [Variable definition](#variable-definition)
   - [Memory Management](#memory-management)
   - [Plugins](#plugins)
   - [Standard Library](#standard-library)
   - [Loops](#loops)
   - [Ranges](#ranges)
   - [Conditions](#conditions)
   - [Types](#types)
   - [Structs](#structs)
   - [Manipulate arrays](#manipulate-arrays)
   - [Functions](#functions)
   - [Include a script file](#include-a-script-file)
   - [Builtin functions](#builtin-functions)
4. [Namespaced aliases](#namespaced-aliases)
   - [Introduction](#namespaced-aliases)
   - [Try it](#try-it)
   - [Available commands](#available-commands)
   - [Shortcuts](#shortcuts)
   - [Environment variables](#environment-variables)
   - [Arguments](#arguments)

<hr>

## Introduction

This project started as a way to put into practice what I learned while reading the Rust programming language book.

It includes features that I find useful, such as a REPL, a calculator,
a scripting language, and a way to store, execute, and load command-line aliases based on the project I'm working on.

Best practices and performance optimization were not a priority, so the code may not be the cleanest or most optimized.

If you would like to contribute, your pull request would be welcome.

### Where the name comes from?

My favorite dish 😋

![image](https://github.com/nbittich/adana/assets/21688838/842c98ca-2c92-4452-93cc-7412685a7a3e)

<hr>

## Installation

1. Docker
   - From the docker hub:
     - `docker run -it nbittich/adana # latest from master`
     - `docker run -it nbittich/adana:0.17.4 # latest release`
   - Manually:
     - clone the repo
     - build the docker image: `docker build -t adana .`
     - `docker run -it adana`
2. Cargo
   - From crate.io:
     - `cargo install adana`
     - `adana`
   - Manually:
     - `cargo build --release`
     - `./target/release/adana`
3. WASM Playground
   - Try it out at https://nbittich.github.io/adana

<hr>

## Programming language

### Getting Started

First, we start with the traditional hello world:

```python
 println("hello world!") # prints hello world
```

In the repl, you could also simply write:

```python
 "hello world!" # prints hello world
```

<hr>

### Comments

Comments are defined like in python, starting with `#`.
You can put them after the last statement or before any useful code, for example:

```python
 # to go to the next line in the repl, press CTRL+x

 # this will be ignored by the repl

 println("hello world!") # this is also ok

```

<hr>

### Multiline

Semicolons are not needed, if you need multiline statements, you can use a multiline block:

```python
fancy_string = multiline {
    "this string\n" +
    "\tis\n" +
    "\t\ton several\n" +
    "lines\n"
}

```

Multiline is useful when you want to process different instructions in several lines:

```python
complex_math_stuff = multiline {
    1 *2
    + 5 *sqrt(2) / 2.
    + 300 / 3
    * 400 % 2 - (1 * 10^3)
}

```

<hr>

### F-Strings

For more complex strings, you can use string blocks / F-Strings.
You can define them using the java syntax:

```java
block_string= """Hello world
I hope you are well.
This is a string block. you can use stuff like "string"
there, nothing will stop you"""
```

Like in javascript, you can add parameters to an f-string:

```javascript
person = struct {
            name  : "nordine",
            wasup : (age) => {
                if (age > 30) {
                    "you are old!"
                } else {
                    "you are young!"
                }
            },
            age  : 34
        }

s1 = """Hello ${person.name}!
You are ${person.age} years old.
${person.wasup(person.age)}"""
```

<hr>

### Operators and constants

There are 22 operators & 3 constants:

| **operator** | **description**  |
| ------------ | ---------------- |
| `+`          | add              |
| `-`          | subtract         |
| `/`          | divide           |
| `*`          | multiply         |
| `%`          | modulo           |
| `^`          | pow              |
| `²`          | pow 2            |
| `³`          | pow 3            |
| `<`          | less than        |
| `>`          | greater than     |
| `<=`         | less or equal    |
| `>=`         | greater or equal |
| `&&`         | and              |
| `\|\|`       | or               |
| `\|`         | bitwise or       |
| `~`          | bitwise not      |
| `@`          | bitwise and      |
| `$`          | bitwise xor      |
| `<<`         | bitwise lshift   |
| `>>`         | bitwise rshift   |
| `==`         | equal            |
| `()`         | parenthesis      |
| `π`          | PI number        |
| `γ`          | EULER number     |
| `τ`          | TAU number       |

Example:

```python
 5 + 5 # 10
 5 + 5.5 # 10.5
 5 / 5 # 1
 5 / 6 # 0
 5 / 6. # 0.8333333333333334 -- we force it to make a float division by adding "."
 5 % 6 # 5 -- modulo on int
 5 % 4.1 # 0.9000000000000004 modulo on double
 5 ^ 5 # 3125
 5 * 5 # 25
 5 * 5.1 # 25.5
 5 * (5+ 1/ (3.1 ^2) * 9) ^3. # 1046.084549281999
 2² # 4
 2³ # 8

```

You can apply an operator before re-assigning a variable, like:

```python
x =2
x+=1 # 3
x-=2 # 1
x*=4 # 4
x%=3 # 1
x/=0.5 # 2


```

It is legal in some circumstances to use the multiply operator implicitly.
It will only work when there is no space between a number (int, decimal) and a variable name.

Example:

```python
x=2
3x²+2x== x*(3x+2) # true
y=0.5x # 1
```

<hr>

### Variable definition

To define a variable, simply type the name of the variable followed by "=".
Variable must always start with a letter and can have numerics or "\_" in it.
Add and assign(+=), subtract and assign (-=), etc are also supported.

```python
 vat = 1.21 # 1.21
 sub_total1 = 10000
 total = vat * sub_total1 # 12100
 sub_total2 = 500 * vat # 605
 sub_total1 = sub_total1 + sub_total2 # 10605
```

It could be simplified as such:

```python
 vat = 1.21 # 1.21
 sub_total1 = 10000
 total = vat * sub_total1 # 12100
 sub_total2 = 500 * vat # 605
 sub_total1 += sub_total2 # 10605

```

It's also possible to use the special variable name "\_" to notify the language that this value
is not used and doesn't have to be stored in context:

```python
_ = 1

for _, n in 1..3 {
   println(n)
}

_ = struct {
   _: "I will not be stored!",
   x: 39
}
```

<hr>

### Memory Management

By default, everything is cloned. As a hobby project, this was a simple way to achieve
more and have fun.

Now that enough features have been built, an attempt to implement automatic-ish
reference counting has started.

This is highly experimental and a partial or full rewrite of the ast / parser may
be needed to implement it properly. To keep the fun working on this, it is not a priority
for now.

You can define a reference as such:

```go
x = 100
y = &x # y points to x, no clone
p = 0
for _ in 0..&x {
  p = p+1
}

```

```go
x = 99
y = &x
x = 100 # now y == 100

```

<hr>

### Plugins

It is possible to load plugins written in rust dyanmically.
Because Rust doesn't have a stable ABI yet, the plugin must be built with the same version that
was used to build adana.

The rust version is specified when running the repl.

To load a library dynamically, you can either specify a relative path, or an
absolute path. In case of a relative path, it should be relative to the
shared lib path (by default: `$HOME/.local/share/adana/db`).

You can override this by providing a path when starting the repl (e.g: `adana -slp /tmp`).

If the path is a directory, it will try to build it using `cargo`, so you
need to have rust installed on your machine.

If it is an `.so` file, it will automatically load it.

An example of plugin can be found in this repo (`dynamic_lib/example_lib_src`).

For example:

- Copy the SO file in tmp: `cp dynamic_lib/libplugin_example.so /tmp/`
- Run and override the lib path: `adana -slp /tmp`
- Execute the following:

```python
     lib = require("libplugin_example.so")
     text = lib.hello("Nordine", "la", "forme?")
```

Or in one line:

```python
   text = require("libplugin_example.so").hello("Nordine", "la", "forme?")
```

<hr>

### Standard Library

A basic standard library exists [here](https://github.com/nbittich/adana-std).

You can use it in this way:

```
fs = require("@std/fs")
fs.api_description() # description of the api

```

If it is not installed yet, you will see instructions on how to install it, e.g:

```
[rust~/toyprograms/adana(master)] fs = require("@std/fs")
std lib doesn't exist: "/home/nbittich/.local/share/adana/lib/adana-std/fs.so".

Try to install it like so:
    - wget -P /tmp https://github.com/nbittich/adana-std/releases/download/0.17.0/adana-std.tar.gz
    - mkdir /home/nbittich/.local/share/adana/lib/adana-std && tar xvzf /tmp/adana-std.tar.gz \
            -C /home/nbittich/.local/share/adana/lib/adana-std
```

### Loops

There are two loops, the while loop and the for-each loop.
The while loop looks like the one in C, while the for-each loop is a little bit more
modern.

For-each loop & while loop don't require parenthesizes.
You can only iterate over structs, strings and arrays.

```C
count = 0

while(count < 10) {
    println(count)
    count = count + 1
   }
```

```
# also valid
while count < 10 {
    println(count)
    count = count + 1
}
```

```javascript
for n in [1,2,3] {
   println(n)
}
```

You have access to the current index in a for-each:

```javascript
for index, n in [1, 2, 3] {
    println("index: " + index + " value: " + n)
}
```

It is also possible to use the for-each loop with a string:

```javascript
for i, letter in "hello" {
  println(i)
}
```

In the case of a struct, the variable will be an entry (a struct with key/value)

```javascript
s = struct {
    name: "nordine",
    age: 34,
    members: ["natalie", "roger","fred"]
}
for  id, entry in s {
     println("Id: "+id +" Key: "+entry.key + " Value: " + to_string(entry.value))
}
```

Parenthesizes are optional for for-each:

```javascript
arr = [1,2,3,4]
total = 0
idx_total = 0
for (index,a in arr) {
 total = total + a
 idx_total = idx_total + index
}

```

You can break if you match a certain condition within a while:

```C
while count < 10 {
     println(count)
     count = count + 1
     if(count % 3 ==0) {
        break
     }
}
```

<hr>

### Ranges

It is possible to define a range like so "start..end" (end exclusive),
or "start..=end" (end inclusive):

```javascript

x = 0..10 # [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
x = 0..=10 # [0,1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

for i in 1..=10 {
  println("print 10 times this message")
}

```

<hr>

### Conditions

Same as C but parenthesizes are optional:

```C
if age > 12  {
    println("age > 12")
} else if age < 9 {
    println("age < 9")
} else {
    println("dunno")
}

```

<hr>

### Types

There is no type-checking in the language. You can add a string to an array, nothing will stop you!

In some cases though, you might get an error.

Below, is a list of types and how you declare them. You can also define your structure.

| **type** | **examples**                                                                                     |
| -------- | ------------------------------------------------------------------------------------------------ |
| null     | `null`                                                                                           |
| bool     | `true` / `false`                                                                                 |
| int      | `5000`                                                                                           |
| u8       | `5`                                                                                              |
| i8       | `-5`                                                                                             |
| double   | `12.` / `12.2`                                                                                   |
| string   | `"hello"`                                                                                        |
| array    | `[1,2,"3", true]`                                                                                |
| function | `() => {"hello"}` <br> `(name) => {"hello" + name}` <br> `(n) => {`<br>&emsp; `"hello"`<br>`  }` |
| struct   | `struct {x: 8, y: ()=> {println("hello!")}}`                                                     |
| error    | `make_err("could not process...")`                                                               |

<hr>

### Structs

You can define structs. Structs are a way of grouping related variables or functions together.
You can define function variables within a struct, but you cannot update the members of the function from within
the struct (there is no `self` or `this`).

The comma is required to separate each member, but not for the latest one.

Example of defining a struct:

```javascript
person = struct {
    name: "hello",
    age: 20
}

person_service = struct {
    say_hi: (person) => { println("hi " + person.name) },
    check_age: (person) => {
             if (person.age < 18) {
                 println("you are too young")
             } else {
                 println("you are too old")
             }
    }
}

person_service.check_age(person)

```

You can access a struct in two ways:

```javascript
name = person["name"] # name contains "hello"
println(person["age"])

age=person.age # age contains "age"
println(person.age)

```

### Manipulate arrays

Arrays are declared like in javascript but are "immutable". After declaration, you cannot (yet) push
new data in them. to do that, you have to concatenate them with another array using the "+" operator.

```python
 arr = [] # declare an empty array
 arr[0] = "kl" # Err: index out of range
 arr = arr + ["kl"] # arr now is ["kl"]
```

You can update a value in the array with the syntax above, as long as the array is greater than the index
provided, e.g:

```python
arr = ["this", "is", "ax", "array", true, 1, 2.3]
arr[2] = "an" #fix the typo
print(arr[2]) # an

```

To get the length of an array, you can use the built-in function `length`

```python
 arr_len = length(arr) # 7

```

Characters within a string can be accessed & updated just like an array:

```python
 s = "this is a strink"
 s[2] # i
 length(s) # 16
 s[15] = "g" # fix the typo
 s # this is a string

```

Here are some other examples of what you can do with arrays:

```python
count = 9
arr = null
# arr = [1, [2, [3, [4, [5, [6, [7, [8, [9, null]]]]]]]]]
while(count > 0) {
    arr = [count, arr]
    count = count -1
}
# prints 1,2,3,4,5,6,7,8,9,done
while(arr != null) {
    print(arr[0] +",")
    arr=arr[1]
}
print("done")

```

<hr>

### Functions

The function can be declared inline or as a block.
In the case of a function parameter, you either assign the function to a variable or
use an anonymous function block.

Parameters cannot be modified within a function. if you want to update something, you have to return it and reassign it.

```python
# no parameters
hello = () => { println("hello, world!") }
hello()

# one parameter
hello_name = (name) => { println("hello "+name) }
hello_name("Bachir")

# takes an array and a function as a parameter
for_each = (arr, consumer) => {
    count = 0
    len = length(arr)
    while(count < len) {
        consumer(arr[count])
        count = count + 1
    }
    return ""  # do not print the count as the repl will print the latest statement
}

for_each(["Mohamed", "Hakim", "Sarah", "Yasmine", "Noah", "Sofia", "Sami"], hello_name)
# or for_each(["Mohamed", "Hakim", "Sarah", "Yasmine", "Noah", "Sofia", "Sami"],
              (name) => { println("hello "+name) }
             )

```

Parameters cannot be modified within a function. if you want to update something, you have to return it and reassign it.
Everything that changes within the scope of a function won't have any effect on the outer scope.

Some other examples of what you can do with functions:

```python
arr  = ["Mohamed", "Hakim", "Sarah", "Yasmine", "Noah", "Sofia", "Sami"]

acc  = (arr, v) => {arr + [v]} # arr is immutable, thus you have to reassign it if you call that function

arr = acc(arr, "Malika")

find_first = (arr, predicate) => {
    len = length(arr)
    count = 0
    while(count < len) {
        temp = arr[count]
        if(predicate(temp)) {
            return temp
        }
        count = count + 1
    }
    return null
}


find_first(arr, (v) => {
    v[0] == "S" || v[0] == "s"
})

# recursive
fact = (n) => {
   if(n>=1) {
    n * fact(n-1)
   }else {
     1
   }
}
fact(10)
```

<hr>

### Include a script file

You can dynamically load a script written in adana in the repl.
Assuming you've cloned the repo and you use docker, here's an example of how to do it.

Note that the extension can be anything.

- map the example directory as a docker volume:

```
docker run -v $PWD/file_tests:/scripts -it adana
```

```python

include("scripts/test_fn.adana") # the built-in function to include
m = map()
m = push_v("nordine", 34, m)
get_v("nordine", m)
```

<hr>

### Builtin functions

There are several built-in functions available.

You already have seen `length` to find the length of an array or string, `include` to include a script inside the repl and `println` to print something.

Here is a list of built-in functions available:

| name        | description                  | example                            |
| ----------- | ---------------------------- | ---------------------------------- |
| sqrt        | square root                  | `sqrt(2)`                          |
| abs         | absolute value               | `abs(-2)`                          |
| log         | logarithm                    | `log(2)`                           |
| ln          | natural logarithm            | `ln(2)`                            |
| length      | length of an array or string | `length("azert")`                  |
| sin         | sine of a number             | `sin(2)`                           |
| cos         | cosine of a number           | `cos(2)`                           |
| tan         | tangent of a number          | `tan(2.2)`                         |
| print       | print without a newline      | `print("hello")`                   |
| println     | print with a newline         | `println("hello")`                 |
| include     | include a script             | `include("scripts/test_fn.adana")` |
| require     | load a shared object         | `require("my_lib.so")`             |
| to_int      | cast to int                  | `to_int("2")`<br>`to_int(2.2)`     |
| to_hex      | format num to hex            | `to_hex(2)`<br>`to_hex(2.2)`       |
| to_binary   | format num to binary         | `to_binary(2)`                     |
| to_double   | cast to double               | `to_double("2.2")`                 |
| to_bool     | cast to bool                 | `to_bool("true")`                  |
| to_string   | cast to string               | `to_string(true)`                  |
| drop        | drop a variable from context | `drop("myvar")`<br>`drop(arr[0])`  |
| eval        | Evaluate a string as code    | `eval("sqrt(9)")`                  |
| type_of     | Type of variable             | `type_of(true)`                    |
| is_u8       | Check if u8                  | `is_u8(0x1)`                       |
| is_i8       | Check if i8                  | `is_i8(-1)`                        |
| is_int      | Check if int                 | `is_int(512)`                      |
| is_double   | Check if double              | `is_double(1.2)`                   |
| is_function | Check if function            | `is_function(()=> {1})`            |
| is_struct   | Check if struct              | `is_struct(struct {})`             |
| is_bool     | Check if bool                | `is_bool(false)`                   |
| is_array    | Check if array               | `is_bool([1,2])`                   |
| is_error    | Check if error               | `is_error(err)`                    |
| make_err    | Create an error              | `make_err("oops")`                 |

<hr>

Note that you can use the repl command `script_ctx` to see what variables are stored in the context.

## Namespaced aliases

You can alias useful commands in a separate namespace (e.g: "work", "git", "docker").

You can then run that command through the repl.
They will be save in disk so you can backup them, restore them etc.

You can also add any kind of values (e.g, ssh keys) to store them.

There is no possible interaction with the scripting language yet.

### Try it

`docker run -it -v $PWD/sample.json:/adanadb.json  nbittich/adana -im`

`restore`

`use misc`

`ds`

`printenv`

### Available commands

| name             | alt        | description                                                                                                                                                |
| ---------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| put              | N/A        | Put a new value to current namespace. can have multiple aliases with option '-a'. e.g `put -a drc -a drcomp docker-compose`                                |
| alias            | N/A        | Alias a key with another. e.g alias commit gc                                                                                                              |
| describe         | ds         | List values within the current namespace.                                                                                                                  |
| listns           | lsns       | List available namespaces.                                                                                                                                 |
| currentns        | currentns  | Print current namespace.                                                                                                                                   |
| backup           | bckp       | Backup the database of namespaces to the current directory                                                                                                 |
| flush            | flush      | Force flush database                                                                                                                                       |
| restore          | N/A        | Restore the database from current directory                                                                                                                |
| deletens         | delns      | Delete namespace or clear current namespace values.                                                                                                        |
| mergens          | merge      | Merge current with a given namespace                                                                                                                       |
| delete           | del        | Remove value from namespace. e.g `del drc`                                                                                                                 |
| get              |            | Get value from namespace. e.g `get drc`                                                                                                                    |
| clip             | clippy     | Get value from namespace and copy it to clipboard. e.g `clip drc`                                                                                          |
| exec             |            | Run a value from the namespace as an OS command. It is completely optional, if you just write the alias, it will also works e.g `exec drc` or simply `drc` |
| cd               |            | Navigate to a directory in the filesystem                                                                                                                  |
| use              |            | Switch to another namespace. default ns is DEFAULT. e.g `use linux`                                                                                        |
| dump             |            | Dump namespace(s) as json. Take an optional parameter, the namespace name. e.g `dump linux`                                                                |
| clear            | cls        | Clear the terminal.                                                                                                                                        |
| print_script_ctx | script_ctx | Print script context                                                                                                                                       |
| store_script_ctx |            | Store script context (optional name) e.g `store_script_ctx 12022023` or `store_script_ctx`                                                                 |
| load_script_ctx  |            | Load script context (optional name) e.g `load_script_ctx 12022023` or `load_script_ctx`                                                                    |
| ast              |            | print ast for script code e.g `ast 9*9`                                                                                                                    |
| help             |            | Display help.                                                                                                                                              |

### Shortcuts

```
CTRL + x => new line in the repl
CTRL + d => quit
CTRL + c => undo
CTRL + l => clear screen
CTRL + r => history search
CTRL + p => π
```

### Environment variables

```
RUST_LOG=adana=debug adana
```

### Arguments

#### TODO NOT EXHAUSTIVE

#### Run a script without entering in the repl

```
# using file
adana -sp /path/to/script.adana

# using code
adana -e 1+1
```

```
# open an in memory db

adana --inmemory

```

```
# override db path & history path + no fallback in memory in case of an error (default to false)
# path must exist! file doesn't have to.

adana --dbpath /tmp/mydb.db --historypath /tmp/myhistory.txt --nofb

```

```
# specify shared lib path
adana -slp /tmp/shared
```

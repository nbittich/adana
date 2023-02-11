# Adana

Scripting programming language, repl and namespaced aliases for commands.

## Goals

 - Making something concrete with rust
 - Learning more about parser combinator
 - Use the minimum amount of libraries
 - Making a scripting language
 - Making a REPL
 - No tutorials, best practices, design patterns, clean architecture, or fancy frameworks

## Table of Contents
1. [Features](#features)
2. [Installation](#installation)
3. [Programming language](#programming-language)
    * [Introduction](#introduction)
    * [Comments](#comments)
    * [Multiline](#multiline)
    * [Operators and constants](#operators-and-constants)
    * [Variable definition](#variable-definition)
    * [Loops](#loops)
    * [Ranges](#ranges)
    * [Conditions](#conditions)
    * [Types](#types)
    * [Structs](#structs)
    * [Manipulate arrays](#manipulate-arrays)
    * [Functions](#functions)
    * [Include a script file](#include-a-script-file)
    * [Builtin functions](#builtin-functions)
4. [Namespaced aliases](#namespaced-aliases)
    * [Introduction](#namespaced-aliases)
    * [Demo](#demo)
    * [Try it](#try-it)
    * [Available commands](#available-commands)
    * [Shortcuts](#shortcuts)
    * [Environment variables](#environment-variables)
    * [Arguments](#arguments)

<hr>

## Features

- alias commands in separate namespaces (dump, merge namespaces, backup, run,...)
- calculator
- simple scripting language
<hr>

## Installation

1. Docker
    * From the docker hub: 
        - `docker run -it nbittich/adana # latest from master` 
        - `docker run -it nbittich/adana:0.13.2 # latest release` 
    * Manually:
        - clone the repo
        - build the docker image: `docker build -t adana .`
        - `docker run -it adana`
2. Cargo
    * From crate.io:
        - `cargo install adana`
        - `adana`
    * Manually:
        - `cargo build --release`
        - `./target/x86_64-unknown-linux-musl/release/adana`

<hr>

## Programming language

### Introduction
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
    "this string" +
    "is " +
    "on several " +
    "lines"
}
```
<hr>

### Operators and constants

There are 14 operators & 3 constants:

|  **operator**  | **description** |
|----------------|-----------------|
| `+`            | add             |
| `-`            | subtract        |
| `/`            | divide          |
| `*`            | multiply        |
| `%`            | modulo          |
| `^`            | pow             |
| `<`            | less than       |
| `>`            | greater than    |
| `<=`           | less or equal   |
| `>=`           | greater or equal|
| `&&`           | and             |
| `\|\|`         | or              |
| `==`           | equal           |
| `()`           | parenthesis     |
| `π`            | PI number       |
| `γ`            | EULER number    |
| `τ`            | TAU number      |


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

```
<hr>

### Variable definition

To define a variable, simply type the name of the variable followed by "=".
Variable must always start with a letter and can have numerics or "_" in it.
Add and assign(+=), subtract and assign (-=), etc are not supported.

```python
 vat = 1.21 # 1.21
 sub_total1 = 10000
 total = vat * sub_total1 # 12100
 sub_total2 = 500 * vat # 605
 sub_total1 = sub_total1 + sub_total2 # 10605
```
<hr>

### Loops

There are two loops, the while loop and the for-each loop.
The while loop looks like the one in C, while the for-each loop is a little bit more
modern.

For-each loop doesn't require parenthesizes. 
You can only iterate over structs, strings and arrays.

```C
count = 0

while(count < 10) {
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
while(count < 10) {
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

Same as C:

```C
if(age > 12) {
    println("age > 12")
} else if(age <9) {
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

| **type** | **examples**                                                                                      |
|----------|---------------------------------------------------------------------------------------------------|
| null     | `null`                                                                                            |
| bool     | `true` / `false`                                                                                  |
| int      | `5`                                                                                               | 
| double   | `12.` / `12.2`                                                                                    |
| string   | `"hello"`                                                                                         |
| array    | `[1,2,"3", true]`                                                                                 |
| function | `() => {"hello"}` <br> `(name) => {"hello" + name}` <br> `(n) => {`<br>&emsp;  `"hello"`<br>`  }` |
| struct   | `struct {x: 8, y: ()=> {println("hello!")}}`
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

You can access a struct in two ways, except for the functions members:

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

You can dynamically load a script in the repl. 
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


| name         | description                                           | example                            |
|--------------|-------------------------------------------------------|------------------------------------|
| sqrt         | square root                                           | `sqrt(2)`                          |
| abs          | absolute value                                        | `abs(-2)`                          |
| log          | logarithm                                             | `log(2)`                           |
| ln           | natural logarithm                                     | `ln(2)`                            |
| length       | length of an array or string                          | `length("azert")`                  |
| sin          | sine of a number                                      | `sin(2)`                           |
| cos          | cosine of a number                                    | `cos(2)`                           |
| tan          | tangent of a number                                   | `tan(2.2)`                         |
| print        | print without a newline                               | `print("hello")`                   |
| println      | print with a newline                                  | `println("hello")`                 |
| include      | include a script                                      | `include("scripts/test_fn.adana")` |
| read_lines   | read a file and returns an array<br> of each lines    | `read_lines("scripts/name.txt")`   |
| to_int       | cast to int                                           | `to_int("2")`<br>`to_int(2.2)`     |
| to_double    | cast to double                                        | `to_double("2.2")`                 |
| to_bool      | cast to bool                                          | `to_bool("true")`                  |
| to_string    | cast to string                                        | `to_string(true)`                  |
| drop         | drop a variable from context                          | `drop("myvar")`                    |
| eval         | Evaluate a string as code                             | `eval("sqrt(9)")`                  |
| type_of      | Type of variable                                      | `type_of(true)`                    |
<hr>

Note that you can use the repl command `script_ctx` to see what variables are stored in the context.

## Namespaced aliases

You can alias useful commands in a separate namespace (e.g: "work", "git", "docker").

You can then run that command through the repl.
They will be save in disk so you can backup them, restore them etc.

You can also add any kind of values (e.g, ssh keys) to store them.

There is no possible interaction with the scripting language yet.

### Demo

https://user-images.githubusercontent.com/3816305/181606658-b01a0ca5-4507-4d58-8625-4ccf9a662b86.mp4



### Try it

`docker run -it -v $PWD/sample.json:/adanadb.json  adana --inmemory`

`restore`

`use misc`

`ds`

`printenv`

### Available commands

| name             | alt        | description                                                                                                                                                                                       |
|------------------|------------|---------------------------------------------------------------------------------------------------------------------------------|
| put              | N/A        | Put a new value to current namespace. can have multiple aliases with option '-a'.  e.g `put -a drc -a drcomp docker-compose`                                                                      |
| describe         | ds         | List values within the current namespace.                                                                                                                                                         |
| listns           | lsns       | List available namespaces.                                                                                                                                                                        |
| currentns        | currentns  | Print current namespace.                                                                                                                                                                          |
| backup           | bckp       | Backup the database of namespaces to the current directory                                                                                                                                        |
| flush            | flush      | Force flush database                                                                                                                                        |
| restore          | N/A        | Restore the database from current directory                                                                                                                                                       |
| deletens         | delns      | Delete namespace or clear current namespace values.                                                                                                                                               |
| mergens          | merge      | Merge current with a given namespace                                                                                                                                                              |
| delete           | del        | Remove value from namespace. Accept either a hashkey or an alias. e.g `del drc`                                                                                                                   |
| get              |            | Get value from namespace. Accept either a hashkey or an alias. e.g `get drc`                                                                                                                      |
| exec             |            | Run a value from the namespace as an OS command.  Accept either a hashkey or an alias. It is completely optional, if you just write the alias, it will also works  e.g `exec drc` or simply `drc` |
| cd               |            | Navigate to a directory in the filesystem                                                                                                                                                         |
| use              |            | Switch to another namespace. default ns is DEFAULT. e.g `use linux`                                                                                                                               |
| dump             |            | Dump namespace(s) as json.  Take an optional parameter, the namespace name. e.g `dump linux`                                                                                                      |
| clear            | cls        | Clear the terminal.                                                                                                                                                                               |
| print_script_ctx | script_ctx | Print script context                                                                                                                                                                              |
| help             |            | Display help.                                                                                                                                                                                     |


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

``` 
# open an in memory db

adana --inmemory

```

```
# override db path & history path + no fallback in memory in case of an error (default to false)
# path must exist! file doesn't have to.

adana --dbpath /tmp/mydb.db --historypath /tmp/myhistory.txt --nofb

```

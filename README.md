# Adana

Toy project with the following goals in mind:

 - Making something concrete with rust
 - Learning more about parser combinator
 - Use the minimum amount of libraries
 - Making a scripting language
 - Making a REPL
 - No tutorials, best practices, design patterns, clean architecture, fancy frameworks

 ## Features

 - alias commands in separate namespaces (dump, merge namespaces, backup, run,...)
 - calculator
 - simple scripting language

 ## Installation

1. Docker
    * From docker hub: 
        - `docker run -it nbittich/adana`
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

## Programming language


First we start with the traditional hello world:

```python
>> println("hello world!") # prints hello world 
```
In the repl, you could also simply write:

```python
>> "hello world!" # prints hello world 
```

### Comments

Comments are defined like in python, starting with `#`.
You can put them after the last statement or before any useful code, for example:

```python
>> # to go to the next line in the repl, press CTRL+x

>> # this will be ignored by the repl

>> println("hello world!") # this is also ok

```

### Semicolon, multiline

Semicolons are not needed, if you need multiline statements, you can use a multiline block:

```python
>> fancy_string = multiline {
    "this string" +
    "is " +
    "on several " +
    "lines"
}
```

### Operators

There are 14 operators:

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
| `||`           | or              |
| `==`           | equal           |
| `()`           | parenthesis     |

```python
>> 5 + 5 # 10
>> 5 + 5.5 # 10.5
>> 5 / 5 # 1
>> 5 / 6 # 0
>> 5 / 6. # 0.8333333333333334 -- we force it to make a float division by adding "." 
>> 5 % 6 # 5 -- modulo on int 
>> 5 % 4.1 # 0.9000000000000004 modulo on double
>> 5 ^ 5 # 3125
>> 5 * 5 # 25
>> 5 * 5.1 # 25.5
>> 5 * (5+ 1/ (3.1 ^2) * 9) ^3. # 1046.084549281999

```

### Variable definition

To define a variable, simply type the name of the variable followed by "=".
Variable must always start by a letter and can have numerics or "_" in it.
Add an assign(+=), subtract and assign (-=), etc are not supported.

```python
>> vat = 1.21 # 1.21
>> sub_total1 = 10000
>> total = vat * sub_total1 # 12100
>> sub_total2 = 500 * vat # 605
>> sub_total1 = sub_total1 + sub_total2 # 10605
```

### Loops

There's only one loop, the while loop. Just like in plain old C:

```C
>> count = 0

>> while(count < 10) {
    println(count)
    count = count + 1
   }
```

You can break if you match a certain condition:

```C
>> while(count < 10) {
     println(count)
     count = count + 1
     if(count % 3 ==0) {
        break
     }
   }
```
### If/else

Same as C:

```C
>> if(age > 12) {
     println("age > 12")
   } else if(age <9) 
   {
     println("age < 9")
   } else 
   {
      println("dunno")
   }

```

### Types

There are no type checking in the language. You can add a string to an array, nothing will stops you!

In some cases though, you might get an error. 

Below, a list of types and how you declare them. You cannot define (yet) your own structure.

| **type** | **examples**                                                                 |
|----------|------------------------------------------------------------------------------|
| null     | `null`                                                                       |
| bool     | `true` / `false`                                                             |
| int      | `5`                                                                          |
| double   | `12.` / `12.2`                                                               |
| string   | `"hello"`                                                                    |
| array    | `[1,2,"3", true]`                                                            |
| function | `() => "hello"` <br> `(name) => "hello" + name` <br> `(n) => {   "hello"  }` |

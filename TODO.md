## Todo 
- stabilize, more tests
- better error
- make a std library
- import/require => like include but scoped context & functions
    - could be like 
    ```
        std = import("/path/to/std")
        std.read_file_to_string("/path/to/file") # call to a function
        pi = std.PI # get a variable
    ```    
- update readme
- less cloning

## In progress

## Done
- <s>Context scope</s>
- <s>break should not be a primitive</s>
- <s>structs</s>
- <s>early return</s>
- <s>file_open = fopen, but try cast each line to the right primitive, default to string</s>
- <s>drop variable</s>
- <s>implement null</s>
- <s>break while</s>
- <s>function & function call</s>
- <s> rename project </s>
- <s>string to array (split? or just each character as a single string in an array)</s>
- <s>variable should start with a letter but can have alphanumeric and maybe _ in it</s>
- <s>array len function</s>
- <s>array / index</s>
- <s>tests array index</s>
- <s>return the Primitive::Error, implement display</s>
- <s>else / else if</s>

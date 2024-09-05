```
store = struct  {todos: [struct {value:"do some work"}, 9]}
       for t in store{
        println(t.key + " " + to_string(t.value)) } # works if the closing bracket is on next line
```

export const EXAMPLES = [
  {
    key: "fizz_buzz",
    label: "Fizz Buzz",
    script: `num = 100
count = 0
text = ""
while count <= num {
    if count % 3 == 0 && count % 5 == 0 {
      # Fizz Buzz
       text = count + " = FizzBuzz"
       println(text)

    } else if count % 5 == 0 {
       # buzz
       text =  count + " = Buzz" # oops
       println(text)
    } else if count % 3 == 0 {
       #fizz
       text = count + " = Fizz"
       println(text)

    } else {
      println(count + " neither fizz nor buzz")
    }
    count += 1
}
`,
  },
  {
    key: "fib",
    label: "Fibonnacci",
    script: `fib = (n) => {
  a = 0 
  b = 1
  c = n 
  x = null
  while  n > 1 {
    n -= 1
    c = a + b
    a = b 
    b = c
  }
  c
}

for i in 0..10 {
  println("""fib(\${i}): \${fib(i)}""")
}
`,
  },
  {
    key: "fact",
    label: "Factorial",
    script: `fact = (n) => {
   if n>=1  {
    n * fact(n-1)
   } else {
     1
   }
}

for i in 0..10 {
  println("""fact(\${i}): \${fact(i)}""")
}`,
  },
  {
    key: "strings",
    label: "Strings",
    script: `v = "eodrnin"
count = 0
# we can iterate over a string
while count < length(v)  {
    println(v[count])
    count = count + 1
}

copy = v

v[0] = copy[6]
v[2] = copy[3]
v[3] = copy[2]
v[4] = copy[5]
v[5] = copy[6]
v[6] = copy[0]

i = 0

s = ""

while i < length("kekeke")  {
    s += "kekeke"[i]
    i += 1
}

println(v)
`,
  },
  {
    key: "sort",
    label: "Sort array of structs",
    script: `# sort array of students
student_1 = struct {
      first_name: "John",
      last_name: "Doe",
      note: 18
}
student_2 = struct {
      first_name: "Jane",
      last_name: "Dow",
      note: 9
}

student_3 = struct {
      first_name: "Bryan",
      last_name: "Bower",
      note: -10
}

students = [student_1, student_2, student_3]

sort = (students) => {
          sorted = false
          while !sorted  {
              sorted = true
              len = length(students)-1
              for i in 0..len {
                  left = students[i]
                  right = students[i+1]
                  if left.note > right.note  {
                       sorted = false
                       students[i] = right
                       students[i+1] = left
                  }
              }
         }
         students
}

println(sort(students))
`,
  },
  {
    key: "arrays",
    label: "Arrays",
    script: `# initial array
arr = ["a", true, "bababa", ("ze" * 4 ), 1, 2.1, 3.]

arr += [69, 420] # append to array

arr *= 2 #  duplicate values in the array

arrlen = length(arr) # length of the array should be 18

copy = []

count = 0

# copy the array one by one 
while count < arrlen {
    copy = copy + [arr[count]]
    count = count + 1
}

println(length(copy) == length(arr))
println("""Are same: \${copy == arr}""")

# mutate array
arr[9]  = "Yolo"
println(arr)

# fancy list
i = 9
list = []

while i !=0 {
    list = [i, list]
    i = i -1
}

# should print [1, [2, [3, [4, [5, [6, [7, [8, [9, []]]]]]]]]]
println(list)

# should print 2
println(length(list))
`,
  },
  {
    key: "maps",
    label: "Key-Value Map",
    script: `map = () => {
    []
}

push_v = (key, value, m) => {
    idx = index_of_v(key, m) 
    if(idx != null) {
        m[idx] = [key, value]
    } else {
        m = [[key, value]] + m
    }
    m
    
}

get_v = (key, m) => {
    res = null
    idx = index_of_v(key,m)
    if(idx != null) {
        k = m[idx]
        res = k[1] 
    }
    res
}

index_of_v = (key, m) => {
    count = 0
    res = null
    while(count < length(m)) {
        k = m[count]
        if(k[0] == key) {
            res = count
            break
        } else {
            count += 1
        }
    }
    res
}

m = map()
m = push_v("nordine", 34, m)
println(get_v("nordine", m))`,
  },
];

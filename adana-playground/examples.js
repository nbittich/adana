export const EXAMPLES = [
  {
    key: "fizz_buzz",
    label: "Fizz Buzz",
    script: `num = 100
count = 0
text = ""
 while count == 0 || count <= num {
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
];

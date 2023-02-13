/*
    - each function call will create a stack frame on the stack
    - each function call has its own stack frame, which contains variables
    - once the execution is over, all the contents of the stack frame will be
      dropped, thi is done by automatically, with the heap we control the data
*/

fn main() {
    // https://blog.thoughtram.io/string-vs-str-in-rust/
    // &      - type reference
    // '      - lifetime notation
    // static - reference that cannot change
    // str    - represents a sequence of bytes (UTF-8)

    // &'static str - is the type of string literals

    let string_literal: &'static str = "String literal.";
    let string_rhythm_of_the_night = String::from("This is the rhythm of the night!");
    let mut string_on_the_heap = String::from("Hello, world!");

    println!("{} {}", string_literal, string_on_the_heap);
    string_on_the_heap.push_str(" Push Me!");

    println!("{}", string_on_the_heap);
    println!("{}", string_rhythm_of_the_night);
}

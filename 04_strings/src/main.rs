/*
    - each function call will create a stack frame on the stack
    - each function call has its own stack frame, which contains variables
    - once the execution is over, all the contents of the stack frame will be
      dropped, thi is done by automatically, with the heap we control the data

    Ownership rules:
    - each value has a variable, that is called the owner of the value
    - there can only be one owner of the value
    - when the owner gets out of the scope, the value will be dropped
*/

fn main() {
    // https://blog.thoughtram.io/string-vs-str-in-rust/
    // &      - type reference
    // '      - lifetime notation
    // static - reference that cannot change
    // str    - represents a sequence of bytes (UTF-8)

    // &'static str - is the type of string literals, they are stored in the binary of the application
    // they are fixed in size and they are available through the lifecycle of the application
    let string_literal: &'static str = "String literal.";

    // this string is stored on the heap
    let string_immutable_on_the_heap = String::from("This is the rhythm of the night!");
    let mut string_on_the_heap = String::from("Hello, world!");

    println!("{} {}", string_literal, string_on_the_heap);
    string_on_the_heap.push_str(" Push Me!");

    println!("{}", string_on_the_heap);
    println!("{}", string_immutable_on_the_heap);

    let s1: String = String::from("hello");
    //let s2: String = s1; // string cannot be s2 will be the owner, and s1 would dangle!
    // remember th 1 owner for a value rule!
    let s2: String = s1.clone();      // to clone/copy a string
    println!("s1={}, s2={}", s1, s2); // this will not work
}

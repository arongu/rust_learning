use std::result;

fn basics() {
    let x = 5;
    println!("x = {}", x);
    let x = "six"; // shadowing
    println!("x = {}", x);

    // scalar data types (they represent a single value)
    // Integers
    let a: i32 = 1000;  // decimal
    let b: i32 = 0xff;  // hexadecimal
    let c: i32 = 0o77;  // octal
    let d: i32 = 0b111; // binary
    let e: u8  = b'A';  // byte (u8 only)

    // Floating point numbers
    let f: f64 = 2.0;
    let g: f32 = 3.0;

    // Booleans
    let h : bool = true;
    let i : bool = false;

    // Character
    let j : char = 'a'; // unicode characters work as well

    // compound data types (represent a group of values)
    // Tuples collection of different type of values
    let tup : (&str, i32) = ("Let's get Rusty!", 1000);
    // data can be access with
    // 1) destructuring will create two 03_basics
    let (channel, sub_count) = tup;
    println!("{}", channel);
    // 2) or by using the index
    let sub_count: i32 = tup.1; // works as indexing in arrays
    println!("{}", sub_count);

    // ARRAYS, arrays are fixed in size. (vectors are dynamic)
    let error_codes: [i32; 3] = [200, 404, 500];
    println!("{}", error_codes[2]);      // this is how you access it
    let eight_zeros : [u32; 8] = [0; 8]; // notice the semi colon!!
}

fn hello() { // Rust uses the snake case convention for function naming
    println!("Hello!");
}

fn print_numbers(x: i32, y: i32) {
    println!("x = {}, y = {}", x, y);
}

fn add_numbers(x: i32, y: i32) -> i32 { x + y }

fn control_flow() {
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 }; // returns 5

    let mut counter = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("counter = {}, result = {}", counter, result);


    let arr:[i32; 4] = [0,1,2,3];
    for n in arr.iter() {
        println!("the value is {}", n);
    }

    for n in 1..10 { // last number is exclusive 0..9
        println!("the value is {}", n);
    }
}

fn main() {
    const MY_CONST: u32 = 1_000_000;
    /* - cannot be mutated
       - must be type annotated
       - constants are literals, they cannot be set from a computed value/function
     */
    basics();
    hello();
    print_numbers(10, 20);
    println!("{}", add_numbers(10, 20));
    control_flow();
}

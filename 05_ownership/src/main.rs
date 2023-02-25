fn main() {
    let x: i32 = 5; //
    let _y: i32 = x; // it works because it has a Copy trait, which allows it to be copied, more on that later
    let z: i32 = 5;

    // ownership changes
    let s0: String = String::from("own_me");
    takes_ownership(s0);
    // println!("s = {}" , s); // this would fail, because the variable does not own anything

    // ownership does not change, because integer has copy trait
    makes_copy(z);
    println!("z = {}", z);

    // gives
    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    // s1 = String::from("whaaaat?"); // immutable variables cannot own new values, but their
                                      // ownership can be revoked by passing it to a function... !!!!
    let s2 = takes_and_gives_ownership(s1);
    println!("s2 = {}", s2);

    // giving "back" ownership to its original owner is only possible when it is mutable
    // references to the rescue! with them you do now have to give ownership back
    let (s3, len) = calculate_length_with_tuple(s2);
    println!("The length of '{}' is {}.", s3, len);

    let s4 = String::from("Calculate my length.");
    println!("Length of '{}' is {}", s4, calculate_length_(&s4));
    println!("s4 = {}", s4); // ownership remains the same

    let mut s5 = String::from("You can change me!");
    change(&mut s5);
    println!("s5 after change = {}", s5);
}

fn makes_copy(some_integer: i32) { // because integer has the copy trait?
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("taking ownership of this String: {}", some_string);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Now you own this string!");
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    println!("I am going to give it back: {}", some_string);
    some_string
}

fn calculate_length_with_tuple(s: String) -> (String, usize) { // this will take ownership as well
    let length: usize = s.len();
    (s,length)
}

fn calculate_length_(s: &String) -> usize { // this will NOT take ownership, called borrowing
    let length: usize = s.len();            // references are immutable by default, so we cannot modify, what we have borrowed
    length
}

fn change(s: &mut String) {
    s.clear();
    s.push_str("I am going to mess up your string!!!!");
}



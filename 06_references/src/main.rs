/*
    1) At any given time you can have either 1 mutable reference or any number of immutable refernce
    2) References must be valid, they cannot dangle!
 */

fn main() {
    //only_one_mutable_reference();
    //mixing_references();
    //borrow_example();
    is_it_move_or_copy();
}

fn borrow_example() {
    let x = 5;
    let borrow_x: &i32 = &x;
    println!("{} {}", x, borrow_x);
}

fn mixing_references() {
    let mut s: String = String::from("This is my string");

    let r1: &String = &s;
    let r2: &String = &s;
    let w1: &mut String = &mut s;
    // println!("{} {} {}", r1, r2, w1); this will not work either
    // if already there is an immutable reference for a value you cannot create a mutable reference
    // for it
    // you can have as many immutable reference as you like
    // the scope is over


    // let r1: &String = &s;
    // let r2: &String = &s;
    // immutable reference scope ends  here therefore you can add:
    // let w1: &mut String = &mut s;
}

fn only_one_mutable_reference() {
    let mut s1 = String::from("Here we go!");

    let _ref_a: &mut String = &mut s1;
    let _ref_b: &mut String = &mut s1;

    //println!("{} {}", _ref_a, _ref_b); // this will crash
}

fn test() {
    let mut s1: String = String::from("Here we go!");
    let r0: &String = &s1;
    // let r1: &String = &s1;
    // let r2: &String = &s1;
    //let w0: &mut String = &mut s1;

    //println!("{} {} {} {}", r0, r1, r2, w0);
    //println!("{} {}", r0, w0);
}

// fn dangle() -> &String { // this will fail as well
//     &String::from("I am dangling.");
// }

fn is_it_move_or_copy() {
    let x = 5;
    let y = x;
    println!("{:p} {:p}", &x, &y); // this is copy

    let s1: String = String::from("mystring"); // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    let s2 = s1;
    println!("{:p} {:p}", &s1, &s2); // this is borrow
}
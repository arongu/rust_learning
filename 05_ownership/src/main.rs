fn main() {
    let x: i32 = 5; //
    let y: i32 = x; // it works because it has a Copy trait, which allows it to be copied, more on that later

    let s: String = String::from("own_me");
    takes_ownership(s);
    // println!("{}" , s); // this would fail, because the variable does not own anything

    // let z: i32 = 5;
    // makes_copy(some_integer: x);
}

fn takes_ownership(some_string:String) {
    println!("owning: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
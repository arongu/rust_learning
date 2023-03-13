fn main() {
    let five = Some(5);
    let six = Some(6);
    let none: Option<i32> = None;

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => ()
    }

    if let Some(3) = some_value { // not exhaustive
        println!("Three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }

    match x {
        Some(i) => Some(i + 1),
        _ => None // any other execute this code
    }
}
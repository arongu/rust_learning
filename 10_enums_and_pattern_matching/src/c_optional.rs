/* builtin enums, in Rust there is no 'null' value
    enum Option<T> {
        Some(T),
        None,
    }
*/

pub(crate) fn optional_examples() {
    let number_string = Some(5); // rust compiler can infer the type
    let some_string = Some("a string"); // same here
    let absent_number: Option<i32> = None; // type is required here
}

pub(crate) fn example2() {
    let x: i8 = 5;
    //let y: Option<i8> = Some(5); // if we change this to
    let y: Option<i8> = None;      // this, it will work

    //let sum = x + y; no implementation for `i8 + Option<i8>`
    let sum = x + y.unwrap_or(0);
    println!("sum = {}", sum);
}

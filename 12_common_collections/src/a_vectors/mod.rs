fn get_from_vec(vec: &Vec<i32>, index: usize) {
    match vec.get(index) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no {} element.", index),
    }
}

pub fn one() {
    let a: [i32; 3] = [1, 2, 3]; // array
    let mut v: Vec<i32> = Vec::new(); // vector created on the heap, it can grow its size
    v.push(1);
    v.push(2);
    v.push(3);

    let v2: Vec<i32> = vec![1, 2, 3]; // macro to create vector from values
    let third: &i32 = &v[2]; // v[20] could be out of bound, because this is a vector, we do not know its size
                             // index 20 can be invalid
    println!("The third element is {}", third);

    // we are using match to be sure what we are accessing is inbound
    match v.get(2) {
        // accessing element in the vector
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    get_from_vec(&v, 1);
    get_from_vec(&v, 10);
}

pub fn two() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i)
    }
}

// enum representing spreadsheet cell with different types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn third() {
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer!"),
    }
}

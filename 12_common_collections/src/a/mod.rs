pub fn one(){
    let a: [i32; 3] = [1, 2, 3];      // array

    let mut v: Vec<i32> = Vec::new(); // it can grow in size
    v.push(1);
    v.push(2);
    v.push(3);

    let v2: Vec<i32> = vec![1, 2, 3]; //
    let third: &i32 = &v[2];
    println!("Thi third element is {}", third);

    v.get(2) {

    }

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    match v.get(10) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }
}

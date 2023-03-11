fn one() {
    let my_string: String = String::from("This is my string");
    let my_slice = &my_string[0..];

    for (i, v) in my_slice.chars().enumerate() {
        println!("index = {}, value = {}", i, v);
    }
}

fn two() {
    let my_string: String = String::from("This is my string!");
    // let my_slice: &str = &my_string[0..my_string.len()];
    let my_slice: &str = &my_string[10..my_string.len()];

    for (index, char) in my_slice.chars().enumerate() {
        println!("index = {}, value = {}", index, char);
    }
}

fn print_range(string: &str, beg: usize, end: usize) {
    if end <= string.len() {
        let my_slice = &string[beg..end];
        for (index, char) in my_slice.chars().enumerate() {
            println!("index = {}, value = '{}'", index, char);
        }
    }
}

fn print_largest(numbers: &[i32]) {
    if !numbers.is_empty() {
        let mut max: i32 = numbers[0];

        for (_, value) in numbers.iter().enumerate() {
            if value.gt(&max) {
                max = *value; //max = i32::from(*value);
            }
        }

        println!("The largest number in the array is: {}", max);
    } else {
        println!("The array is empty.");
    }
}

fn main() {
    let my_string = String::from("This is my string");
    print_range(&my_string, 0, 0);
    print_range(&my_string, 0, my_string.len());

    let my_numbers: [i32; 4] = [10, 88, 1, -12];
    print_largest(&my_numbers);
}

fn main() {
    // let mut s: String = String::from("hello world");
    // let hello = &s[0..5]; // &s[..5]  // slices do not take ownership
    //let world = &s[6..11];   // &s[6..]  // slices do not take ownership
    //let world = &s[..];   // &s[..]

    //let word = first_word_improved(&s);
    // s.clear();
    // println!("{}", word);

    let mut s: String = String::from("hello world");
    let s2 = "hello world"; // string literals are string slices in the binary

    let word = first_word_improved(s2);
}

fn first_word_improved(s: &str) -> &str { // works with &String and &str, automatically get coerced to a string slice
    let bytes= s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn first_word(s: &String) -> usize {
    let bytes= s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
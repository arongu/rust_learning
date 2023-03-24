// strings
// ascii is 1 byte - 7bit represents the character - english letters and few symbols
// different languages have different encoding
// rust uses therefore utf-8
// utf-8 is a varying length type, meaning a character can be stored
// in 1,2,3,4 bytes
// the first 127 symbols of utf-8 is same as ascii, this make is backward compatible with ascii
pub fn one() {
    let s1 = String::new(); // empty immutable new
    let s2: &str = "string slice";
    let s3: String = s2.to_string();
    let s4 = String::from("string slice"); // from string slice
    // to be continued

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push('!');

    println!("{}", s5);
}

pub fn two() {
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3: String = s1 + &s2;

    //println!("{} {}", s1, s2); // would not compile because s1 is moved
    println!("{} {}", s3, s2);
}

fn main() {
    //only_one_mutable_reference();

}

fn only_one_mutable_reference() {
    let mut s1 = String::from("Here we go!");

    let _ref_a: &mut String = &mut s1;
    let _ref_b: &mut String = &mut s1;

    //println!("{} {}", _ref_a, _ref_b); // this will crash
}

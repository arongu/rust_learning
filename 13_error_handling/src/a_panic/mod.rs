pub fn panic() {
    panic!("crash and burn");
    //println!("Do we reach this?");
}

pub fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Do not call me with 22!");
    }
}

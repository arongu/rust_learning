mod sub_a;
mod elneveztem;

pub fn hello(){
    println!("a::Hello!");
    sub_a::hello();
    elneveztem::hello();
}

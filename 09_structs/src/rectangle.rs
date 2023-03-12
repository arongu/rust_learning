/*
// re-writing the rectangle program to use structure
    let width1:u32 = 30;
    let width2:u32 = 50;

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
*/

// ------ part 1 --------
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

pub(crate) fn one() {
    // user::one();
    // struct Color(i32, i32, i32); // tupled structures
    // struct Point(i32, i32, i32);
    let rect:(u32,u32) = (30, 50);

    println!("The area of rectangles is {} square pixels.", area_tuple(rect));
}


// ------ part 2 --------
#[derive(Debug)] // this is a trait
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // self which is the instance which the method is being called
    fn area(&self) -> u32 {
        self.height * self.width // the syntax is the same for reference or an object, rust has a feature 'automatic de-referencing'
                                 // c++ x -> area(); x.area() -- no need for this in here
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        //self.area() >= other.area()
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle { // associated function, you do not pass &self
        Rectangle {
            width: size,
            height: size
        }
    }
}

// this can be added as a method
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub(crate) fn two() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("Rectangle {:#?}.", rect);
}

pub(crate) fn three() {
    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    let3 = Rectangle::square(10);

    println!("react1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("react2 can hold rect1: {}", rect2.can_hold(&rect1));
}

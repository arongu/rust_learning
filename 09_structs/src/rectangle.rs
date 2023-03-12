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
    /*
        In C and C++, two different operators are used for calling methods: you use .
        if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object
        and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().
        Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing.
        Calling methods is one of the few places in Rust that has this behavior.
     */
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        //self.area() >= other.area()
        self.width > other.width && self.height > other.height
    }

    fn myself(self : &Rectangle) { // &self is shorthand for self : &Self and &Self is an alias for the current type
        //                         // in this case &Rectangle
        println!("Look at me height={} width={} area={}", self.height, self.width, self.area());
    }

    /*
        &self             -- it represents itself
        self : &Self      -- use the &Self alias which always represents the type of itself
        self : &Rectangle -- ditch the alias and use the type itself
     */
}

/*
    All functions defined within an impl block are called associated functions because they’re associated
    with the type named after the impl.
    We can define associated functions that don’t have self as their first parameter (and thus are not methods) !!!
    because they don’t need an instance of the type to work with.

    We’ve already used one function like this: the String::from function that’s defined on the String type.
    Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    These are often called new, but new isn’t a special name and isn’t built into the language.
    For example, we could choose to provide an associated function named square that would have one dimension parameter
    and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:
 */

// multiple impl blocks are allowed!
impl Rectangle {
    fn square(size: u32) -> Rectangle { // associated function, this is not a method, they are usually used as a "constructor"
        Rectangle {
            width: size,
            height: size
        }
    }

    fn square_self(size: u32) -> Self { // using the Self alias as a type, this is not a method
        Self {
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

    let rect3 = Rectangle::square(10);

    println!("react1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("react2 can hold rect1: {}", rect2.can_hold(&rect1));
    rect3.myself();
}

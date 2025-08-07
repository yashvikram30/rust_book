#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {

    fn area (&self) -> u32 {
        self.width * self.length
    }
    // Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle that is also named width

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size:u32) -> Self{

        Self{
            width: size,
            length: size
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        length: 40
    };

    println!("The area is: {}",rect1.area());

     /*
    Here, we’re choosing to make the width method return true if the value in the instance’s width field is greater than 0 and false if the value is 0: we can use a field within a method of the same name for any purpose. 
    In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. When we don’t use parentheses, Rust knows we mean the field width.
     */
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // using can_hold function
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        length: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /*
    We can define associated functions as functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with
     */
    let sq = Rectangle::square(3); // associated functions are called in this manner

    // method calls are syntactic sugar for function calls
}


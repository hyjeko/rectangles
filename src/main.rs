//let us see a pretty print of this struct, derived from debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
To define the function within the context of Rectangle,
we start an impl (implementation) block for Rectangle.

Methods can take ownership of self,
borrow self immutably as we’ve done here, or borrow self mutably,
just as they can any other parameter.

Having a method that takes ownership of the instance by using just 'self'
as the first parameter is rare

All functions defined within an impl block are called associated functions
because they’re associated with the type named after the impl
*/
impl Rectangle {
    /*
    Methods are different from functions in that
    they’re defined within the context of a struct
    (or an enum or a trait objec`t), and their
    first parameter is always self

    &self indicates we want to borrow the reference (don't descope rect1)
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        // return self.width > 0; (if you want to use a semicolon)
        self.width > 0
    }

    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
    We can define associated functions that don’t have self as
    their first parameter (and thus are not methods)
    because they don’t need an instance of the type to work with.
    We’ve already used one function like this, the String::from function,
    that’s defined on the String type.

    To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3);
    This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces
    created by modules. We’ll discuss modules in Chapter 7.
    */
    fn _square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

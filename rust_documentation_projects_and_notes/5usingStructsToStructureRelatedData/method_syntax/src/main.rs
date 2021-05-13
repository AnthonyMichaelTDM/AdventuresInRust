#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
/*To define the function within the context of Rectangle, we start an impl (implementation) block*/
impl Rectangle {
    //this is a method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //this is a method with more than one parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //this is a an associated method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*Methods are similar to functions: they’re declared with the fn keyword and their name, 
    they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. 
    However, methods are different from functions in that they’re defined within the context of a struct 
    (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), 
    and their first parameter is always self, which represents the instance of the struct the method is being called on.*/

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(51); //you call associated functions with :: syntax

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); //call methods with dot syntax
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );



    /*Structs let you create custom types that are meaningful for your domain. 
    By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. 
    Methods let you specify the behavior that instances of your structs have, 
    and associated functions let you namespace functionality that is particular to your struct without having an instance available.

    But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.*/

}

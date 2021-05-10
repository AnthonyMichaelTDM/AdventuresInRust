// this program will show 3 progressively better ways, heigh width, find area
fn main() {
    //basic way, not great bc width and height are related values, we can group them in 2 ways
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_basic(width1, height1)
    );

    //uses tuples, but what if we want the 2 integers to have more meaning?
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    //uses a struct, this is good
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );


    //adding useful functionality with derived traits
    println!("rect1 is {:?}", rect1);
}
#[derive(Debug)] //needed to let us print out the struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_basic(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}




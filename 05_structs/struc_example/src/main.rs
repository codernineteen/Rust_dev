#[derive(Debug)] // This enables us to debug our print
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    }; //ownnership is binded to main scope

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) //borrow
    );

    //println! macro's default formatter is {} curly braces
    //But, println! macro has various display possibilities included above.
    //Struct can't be printed with default formatter.
    println!("{:?}", rect1); //debug formatter.
    println!("{:#?}", rect1); //When we have a larger struct, this useful.

    //Alternative to print out a value using the Debug format
    //this takes ownership of expression if you don't use references.
    dbg!(&rect1);
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

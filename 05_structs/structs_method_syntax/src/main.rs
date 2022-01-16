#[derive(Debug)]
//Methods is like a function
//but only difference is it is defined within context of structure.

struct Rectangle {
    width: u32,
    height: u32,
}

//#1
//first parameter always 'self'
//All functions in impl block are called 'associated function'
impl Rectangle {
    //instead of rectangle: &Rectangle, we just use &self.
    //self of type 'Self' borrows 'Self' instance.
    //you can also take ownership, borrow (immutably, mutalbly) like function.
    fn _area(&self) -> u32 {
        self.width * self.height
    }

    //#2
    //methods with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //#3
    //associtated functions that aren't methods doens't include self paramter
    //This can be used for creating new instance.
    //Think about String::from("string")
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //#2
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    //#3
    let square1 = Rectangle::square(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("{:#?}", square1);
}

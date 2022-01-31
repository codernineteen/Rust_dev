use std::ops::Add;
//Associated types connect a type placeholder
//the trait method definitions can use these placeholder types in their signatures
//Implementors of the Iterator trait will specify the concrete type for Item
//e.g Iterator trait -> implementor(Couter) -> item of Counter -> return Option<counter item>
pub trait Iterator {
    //Type 'Item' is a placeholder type
    type Item;
    //next method returns Option with Item type
    fn next(&mut self) -> Option<Self::Item>;
}

//When trait has a generic parameter, it can be implemented for a type multiple times
pub trait IteratorGeneric<T> {
    //Because trait has a generic paramneter,
    //Whenver we implement this method, we should specify type of generic parameter
    fn next(&mut self) -> Option<T>;
}
//Associated types don't need to be annotated because we can't implement a trait on a type multiple times

//#1
//default generic type parameters and operator overloading
//<placeholder type=concrete type>
//operator overloading example
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

//from std::ops::Add
//Rhs=Self syntax is called default type parameters(Rhs : right hand side)
///If we don't specify a concrete type for Rhs, it will be 'Self' as default
// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

//Here Rhs type will be Self because we don't specify
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//Let's customize Rhs
struct Millimeters(u32); //newtype
struct Meters(u32); //newtype

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    //Once we wrote a function parameter of type Meters, we can't use Millimeters or plain u32 as a parameter
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

//#2
//Rust neither prevent a trait from having a method with the same name as another trait's method
//nor prevent you from implementing both traits on one type
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your caption speaking")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("waving arms furiously")
    }
}

//fully qualified syntax
trait Animal {
    fn baby_name() -> String;
}

struct Dog;
//associated function
impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}
//trait implementation
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

//#3
//using supertraits to require one trait's functionality within another trait
use std::fmt;

//Outlineprint requires Display trait (fmt::Display)
//we can use to_string without adding a colon and specifying Display trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PointTwo {
    x: i32,
    y: i32,
}

//Implement Display trait first
impl fmt::Display for PointTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
//then implement OutlinePrint trait which use Display trait
impl OutlinePrint for PointTwo {}

//#4
//Newtype pattern to implement external traits on external types
//To implement Display trait for Vec<T>, wrap Vec<T> with struct
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //access item with index 0 -> now we can treat Wrapper struct like Vec<T>
        //The downside of this newtype is here,
        //To use methods related to Vec<T>, we always need to extract item from tuple(self.0)
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    //#2
    let person = Human;
    //this code print "waving arms furiously"
    person.fly();
    // from Pilot trait
    Pilot::fly(&person);
    // from Wizard trait
    Wizard::fly(&person);

    //To disambiguate, we should use fully qualified syntax
    //we provide a type annotiation within the angle brackets
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    //#4
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

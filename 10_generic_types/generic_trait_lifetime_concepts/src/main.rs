//generics are abstract stand-ins for concrete types or other properities
//Option<T>, Vec<T>, HasnMap<K,V> and Result<T, E> are generics example

fn main() {
    println!("Hello, world!");
}

//#1
//Removing duplication of functions
//Instead of repeating same logics for different input, we can create an abstraction.
//this fn represents any concrete slice of i32
fn largest_concrete(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
//The step is : 1. identify duplication -> 2. extract logic and create new abstraction -> call the abstraction(function)

//#2
//Generic data types
//above function is limited to type 'i32'
//more generic cases, we can use parameterize.
//By convention, parameter type is 'T' as you already saw

//a. function
//Declare parameter name in front of parenthesis and use it in there.
fn largest_generic<T>(list: &[T]) {}

//b. struct
struct Point<T> {
    //both x and y have generic type
    x: T,
    y: T,
}
//By the way, We use only one generic type 'T' in here,
//so If you use like this, let error_prone = Point {x:0, y:1.0}; -> this cause compiling error.
//If you want to use different generic types in Struct? create another one
struct PointTwo<T, U> {
    x: T,
    y: U,
}
// Now, let error_prone = Point {x:0, y:1.0}; -> this works

//c. enum
//Look how Option<T> and Result<T, E> are structured.
enum Option<T> {
    //two variants : Some and None
    Some(T), //Some takes a value of generic type 'T'
    None,
}

enum Result<T, E> {
    //two variants : Ok and Err
    Ok(T),  //Ok takes a value of generic type 'T'
    Err(E), //Err takes a value of generic type 'E'
}

//#3
//method definitions
//we have to declare <T> right after impl to specify we implement this method to Point<T>
//T of impl<T> and T of Point<T> is different, but using same name is conventional.

impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self.x
    }
}
//Or we can define method with constraints
//This only applies to Point<f32> with f32 type.
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//This shows a situation in which some generic parameter are declared with impl
//and some are declared with the method definition.
impl<T, U> PointTwo<T, U> {
    //self from PointTwo<T, U>
    //Return x point of T , y point of U1
    fn _mix_up<T1, U1>(self, other: PointTwo<T1, U1>) -> PointTwo<T, U1> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

//The good thing is there is no runtime cost of using generic code

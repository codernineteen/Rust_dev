fn main() {
    //#1
    //match
    //match expression need to be exhaustive (you should care all possibilities)
    //a particular pattern '_' will match anything, but can't bind value
    let coin = Coin::Penny;
    match coin {
        Penny => println!("this is penny"),
        Others => println!("this is others"),
    }

    //#2
    //conditional if let
    //more flexible because we can express the way we want
    //you can express arms like - if let, else if, else if let
    let fv_color: Option<&str> = Some("red");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fv_color {
        println!("ur favorite color is {}", color)
    } else if is_tuesday {
        println!("It's tuesday")
    } else if let Ok(age) = age {
        //you can use shadowed value like match
        if age > 30 {
            println!("you are still young")
        } else {
            println!("you are so young")
        }
    } else {
        println!("you need favorite color")
    }
    //But it is also downside of if let expresssions that compiler doens't check exhausetiveness.
    //Because of this fact, there is a possiblitiy of bug in logic

    //#3
    //while let conditional loop
    let mut stack = vec![1, 2, 3];
    //pop return Option<T> from top of stack
    while let Some(top) = stack.pop() {
        println!("{}", top);
    } //when pop returns None, goes out of scope

    //#4
    //for loop
    //In the for loop, the pattern is the value that direcly follows the keyword
    //e.g. for x in uy -> x is pattern
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    //#5
    //let statement
    //let PATTERN = EXPRESSION;
    let x = 5; //a variable name is a simple form of pattern
    let (x, y, z) = (1, 2, 3); // x -> 1 , y -> 2 , z -> 3
                               // let (x, y) = (1,2,3) -> this won't work
}

//#6
//function parameters
//samely, x is pattern whose type is i32
fn foo(x: i32) {
    //code
}

enum Coin {
    Penny,
    Nickel,
}

//normal declartion
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //constants 
    //all uppercase, underscore between words, can use any scope, never can mutate
    const PRIVATE_NUMBER: u32 = 7;

    //shadowing
    let spaces = "    ";
    let spaces = spaces.len(); //we can change type! awesome!
}




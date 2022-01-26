fn main() {
    //example of irrefutabilitiy
    let x = Some(5); //x can match anything and therefore cannot fail to match

    //example of refutability
    //if let -> can fail to match
    //irrefutable -> let, for loop
    //refutable -> if let, while let

    //For example, let Some(x) = some_option_value; can't be compiled
    //We can change this code like
    if let Some(x) = x {
        println!("{}", x)
    }
}

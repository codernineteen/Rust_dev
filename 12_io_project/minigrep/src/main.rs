//This project use Closures and Iterators concept. Refer to Chapter 13 directory.
//short iterator concetps
//1. iterators produce a series of values
//2. collect method on an iterator -> turn iterator it into a collection(e.g Vec<T>)

//This library enables our code to read any command line arguments.
use minigrep::Config;
use std::env; //rather than bring a specific function 'args', we just use parent module!
use std::process;

//Four problems you need to solve
//1. when code has more responsibilities, It's harder to read, to solve problem
//2. The more variables we have, the more it becomes difficult.
//3. you should give more specific error description, not just like 'something went wrong'
//4. using expect whenever there may be an error can be verbose

//Solution
//1. split functionilities so that each functions can take one responsibility
// follow below steps
// a. split your program into a main.rs and a lib.rs . lib.rs manage your program logic
// b. if a logic is too small, you can maintain it in main.rs
// responsibilities of main function is : calling command line parsing logic, setting up configuration, calling a run function in lib.rs , handling error of run function
//2. grouping configuration values - structure variables and make those meaningful
//3. improve error description with custom error messages.

fn main() {
    //handle error in a more user-friendly way
    //env.args() return iterator
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //eprintln! macro that prints to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //we can check whether 'run' return Err or not with if let syntax
    //we don't need to care about Ok case because it is just unit type
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    //To get to know whether it returns a Result. we can choose two ways
    //1. test with compiler
    //2. look at the standard library API

    //#1
    //File::open might be successful or might not have permission to access the file.
    //we can divide these two cases with Result type.
    //success -> will be instance of Ok
    //fail -> Err
    let f = File::open("hello.txt");
    //To check what instance f has.
    let f0 = match f {
        Ok(file) => file,
        //we can also handle different kind of errors.
        Err(error) => match error.kind() {
            //if error kind is notfound, we try creating file
            ErrorKind::NotFound => match File::create("hello.txt") {
                //if it is successful
                Ok(fc) => fc,
                //if it is fail
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    //#2
    //shorcut for panic on Error.(unwrap and expect)
    //Result<T,E> type has many helper methods.
    //Regarding to unwrap, if Result Ok -> Ok / if Result Err -> call panic.
    //unwrap
    let f1 = File::open("helloTwo.txt").unwrap();
    //expect , works same way. -> you can customize error message
    let f2 = File::open("helloTwo.txt").expect("Failed to open helloTwo.txt");
}

//#3
//propagating errors
//Return type is Result<String, io::Error> : generic parameter T -> String, generic parameter E -> io::Error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//#4
//shorcut of above code.
//use ?(question mark) operator
//The difference is : error with ? -> from function(defined in From trait) -> error type = return type of function
fn read_username_from_file_shorcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; //return value inside Ok to the variable f , if Error, give any Err value to the calling code.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) //return type
}

//you can't mix and match Between Result and Option
//you only can use ? operator for one Type

//#5
//return type of main
//1.integer / success -> () -> 0 , error -> non zero -> panic
//2. Result<(), E>
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }

//#6
//use case
// unrecoverable -> no way to handler
// recoverable -> Result type -> calling code choose recoverable way or unrecoverable way(panic)

//unwrap, expect are very handy for prototyping (before you decide how to handle error)
//If you have more information than compiler, go with unwrap
//If it is interaction with user, it is better with Result type

//#7
//guide line for error handling

//bad state(e.g invalid input) -> panic!
//depends an external code which is out of your control -> panic!
//expects failure -> return a Result type(e.g http request and status code).
//operation on values -> 1. verify value first -> 2. if value invalid, panic
//use a typed parameter in your function reduce verbose error hadnlings.

//#8
//create new type to avoid tedious error handling.
//Assume that you want user to guess a number between 1 and 100
//create custom type named 'guess'
pub struct Guess {
    value: i32,
}

impl Guess {
    //create new value
    pub fn new(value: i32) -> Guess {
        //if value is out of range, call panic!
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        //if value lives in range, return new instance of Guess
        Guess { value }
    }

    //getter
    pub fn value(&self) -> i32 {
        self.value
    }
}

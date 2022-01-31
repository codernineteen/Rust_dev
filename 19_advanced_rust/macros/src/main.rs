//A term 'macro' is composed of declaritive macros with macro_rules!
//and three kinds of procedural macros
//1. Custom #[derive] macros (specify code added with derive attribute used on structs and enums)
//2. Attribute-like macros that define custom attributes usable on any item
//3. Function-like macros that look like function calls but operate on the tokens specified as their argument

//#1
//Why we need macros?
//Macros are a way of writing code that writes other code(metaprogramming)
//Unlike functions, macros can take a variable number of parameters
//e.g println!("hello") -> one argument, println!("{}", hello) -> two arguments
//Also, macros are expanded before the compiler interpret the meaning of code
//while function gets called at runtime

//The important thing is you must define macros or bring them into scope before you call them in a file
//while you can define function anywhere and call anywhere

//#2
//Declarative macros with 'macro_rules!'
//Macros compare a value(literal source code) to patterns -> when matched, replaces source code to the macro
//e.g. simplifed version of vec! macro

//we can use this macro in another scope by using this annotation
#[macro_export]
//start macro definition with macro_rules!
//macro name without exclamation mark
macro_rules! vec {
    //macro body
    //body structure is simlar to match expression
    //Here one arm : ($($x: expr), *)

    //() -> first parentheses encompass the whole pattern
    //$ -> match the pattern within the parentheses
    //$x: expr -> matches any Rust expression and gives the expression name '$x'
    //, -> a literal comma separator character could optionally appear
    //* -> specifies that the pattern matches zero or more of whatever precedes the *
    ($($x:expr), *) => {
        //if the pattern matched, this associated code block will be emitted
        {
            let mut temp_vec = Vec::new();
            //$()* is generated for each part that matches $() depending on how many times the pattern matches
            //$x is replaced with each expression matched
            $(
                temp_vec.push($x);
                //if vec![1,2,3]
                //temp_vec.push(1);
                //temp_vec.push(2);
                //temp_vec.push(3);
            )*
            temp_vec
        }
    };
}

//#2
//procedural Macros
//accept some code as an input , operate on that code, and produce some code as an output
//when creating this kind of macros, the definition must reside in their own crate

// 1. custom derive macro
//check hello_macro dir

fn main() {
    println!("Hello, world!");
}

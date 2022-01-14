fn main() {
    println!("Hello, world!");
    compare_statement_expression();
    try_return();
}
//rust use snake_case for function name
//declaration order is not important
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
// you must declare the type of each parameter.

//statement do not return value, Expression evaluate to a resulting value.
//calling a function, calling a macro is and expression.
fn compare_statement_expression() {
    print_labeled_measurement(-5, 'h'); // this is expression
    //this scope is also expression
    let expression = {
        let statement = 3; // this is statement, does not return anything.
        statement+1 // return, no semicolon
    };
    println!("{}", expression)
}

// returnning use arrows style, the last expression will be return value;
// returning value need type
fn try_return() -> i32 {
    5
}
//fn - function pointer
//Fn - closure trait
fn add_one(x: i32) -> i32 {
    x + 1
}
//take a function as a parameter
//Unlike closures, fn is a type rather than a trait
//function pointers implement all three clousre traits(Fn, FnMut and FnOnce)

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    //#1
    //It's best to write functions using a generic type and one of the closure traits
    //to accept either functions or closures
    //e.g. map method
    //closusre parameter
    let num_list = vec![1, 2, 3];
    let string_list: Vec<String> = num_list.iter().map(|i| i.to_string()).collect();
    //or named function
    //also we used fully qualified syntax here(ToString::to_string) because there are some methods whose name is to_string
    let string_list_func: Vec<String> = num_list.iter().map(ToString::to_string).collect();

    //another pattern
    enum Status {
        Value(u32),
        Stop,
    }
    //Status::Value instance
    let status_list: Vec<Status> = (0u32..20).map(Status::Value).collect();

    //#2
    //closures are represented by traits -> you can't return closures directly
    //To return closure, we need to specify its size to compiler
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

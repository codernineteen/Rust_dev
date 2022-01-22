use std::ops::Deref;

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    //we can dereference Box<T>.
    assert_eq!(5, *y);

    let z = MyBox::new(x);
    //Both are possible . you don't need to worry about whether you need to call deref method.
    assert_eq!(5, *z);
    assert_eq!(5, *(z.deref()));

    //deref coercion
    let m = MyBox::new(String::from("Rust")); // MyBox<String> -> deref method call -> &String
    hello(&m); // &String -> deref method call -> String -> coercion -> str -> &str
               //What if deref coercion didn't exist?
    hello(&(*m)[..]);
}

//Custom Box<T>
struct MyBox<T>(T); //we store T as tuple

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    //This syntax defines associated type for the deref trait to use
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 //destructuring tuple and return reference of that value to dereference
                //(T) -> self.0 -> T -> &self.0 -> &T (now can dereference)
                //If this method return a value directly, not reference, value will be moved out of self.
    }
}

//Deref coercion
//This only works on types that implement the Deref trait (such type -> a reference to another type)
//e.g. &String(has Deref trait) -> deref coercion -> &str
//A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

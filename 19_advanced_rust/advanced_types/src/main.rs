use std::fmt;

fn main() {
    //#1
    //type synonyms with Type Aliases
    //Now the alias kilometers is a synonym for i32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
    //The main usage for type synonyms is to reduce repetition
    //Maybe we have a lengthy type like this
    type Thunk = Box<dyn Fn() + Send + 'static>;
    //We can reduce repetition
    fn takes_long_type(f: Thunk) {}
    //Type aliases are also commonly used with the Result<T,E> type
    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
    //Type alias makes code easier to write and gives us a consistent interface

    //#2
    //The Never type that never returns
    //Rust have a special type named '!' = empty type = never type
    //e.g.
    //A function that returns never like this is called diverging function
    fn bar() -> ! {
        //--snip--
        loop {
            //continue has a ! value
            continue;
        }
    }

    //The never type is useful with the panic! macro as well
    //panic! has the type !
    //Think about unwrap method which returns type T or panic! according to match expression
    //The result of the overall match expression is T because panic! doesn't produce a value

    //#3
    //Dynamically sized Types and the sized trait(DST, unsized types)
    //these types let us write code using values whose size we can know only at runtime
    //Let's think about 'str' type , not '&str'
    // let s1: str = "Hello there!";
    // let s2: str = "Hello there!";
    //unfortunately these two line of code will cause error
    //because Rust needs to know how much memory to allocate for any value of a particular type
    //and all values of a type must use the same amount of memory

    //Alternatively, as we already know, we can use &str instead
    //&str is two values: the address of the str and its length
    //The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

    //To work with DSTs, Rust has a particular trait called Sized trait
    //This trait is automatically implemented for everything whose size is known at compile time
    //But we can relax this restriction

    //Notice that we used &T because now T may or may not be sized as to ?Sized
    fn generic<T: ?Sized>(t: &T) {
        //snip
    }
}

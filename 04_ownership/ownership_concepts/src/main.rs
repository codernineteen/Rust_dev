//ownership : This is how rust manages garbage memory.
//python, javascript : garbage collector
//c, c++ : allocation and free the memory
//Rust : ownership rules. If not fit into rule, compiler won't process.

//Additional important concepts for understand Ownership : Stack and heap
//Stack : last in (push), first out(pop) - should be known and fixed size.
//heap : request a certain amount of data space -> memory allocator finds an empty space , marks it as in use -> returns pointer (address of location)
//speed : stack(no need allocator) > heap
//purpose of ownership is to manage heap.

//ownership rule : each value has a variable(owner), one owner at a time, owner goes out scope -> value dropped
fn main() {
    //#1 - scope rule
    {
        // invalid - not declared
        let s = "hello"; // valid from now on
    } // invalid - out of scope

    //#2 - ownership example with string types
    let mut s = String::from("hello"); // create a String - String type, this can be mutated but literals not
                                       //literal hardcoded, immutable
                                       //String mutable, growable -> unknown size -> this type needed to request allocator.
                                       //As a result, String::from requests the memory it needs
                                       //when memory doesn't need , rust drop it with 'drop' func automatically
}

//#3
//In more complicated situations, it goes more complex.
fn multi_variables() {
    let x = 5;
    let y = x; // this is evident. intergers are known, fixed size -> go onto stack
    {
        let s1 = String::from("hello");
        // s1 is composed of pointer(which points to heap), length: 5, capacity: 5
        let s2 = s1;
        // when s2 copy it, it only copy ptr, len, cap (not data on the heap)
    }
    //here probem comes out, when both s1 and s2 goes out of scope together -> they share pointer -> drop function try dropping same memory (double free error)
    //To prevent this error, rust regards s1 as invalid value if s2 copy s1.
    //and this kind of data copying , known as 'move' in rust.
}

//#4
//If you want to copy deeply, use clone method. clone method copy even heap data.
fn deep_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); //expensive behavior when your data is big.
}

//#5
//stack-only data
fn stack_only() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    //when copying known size data, there is no difference between just copying like this and clone method.
}

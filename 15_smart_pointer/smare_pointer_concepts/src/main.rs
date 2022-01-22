use crate::List::{Cons, Nil}; //absolute path import

//pointer : a variable that contains an address in memory
//references are pointers that only borrow data

//smart pointer : data structure (address + additional metadata and capabilites)
//smart pointers own the data they point to.

fn main() {
    println!("Hello, world!");
    store_in_box(5);

    //At a compile time, the size is i32 + Box size
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

//#Box<T>
//Boxes allow you to store data on the heap rather than the stack.
//The situations you use box
// - size of a type is unknown at compile time and you want to use it as exact size
// - when you want to transfer ownership of large amount data which doesn't have Copy trait
// - Own a value whose type implements a particular trait

fn store_in_box(v: i32) {
    //allocate data on the heap
    let b = Box::new(v); //By the way, storing a single value in Box is not useful
    println!("{}", b);
} //deallocation for box and the data stored on the heap

//cons list : a data structure
//cons function constructs a new pair from its two arguments(a single value and another pair)
//These pairs which contains pairs form a list (cons list)
//cons list contains two elements(the value of the current item and the next item)
//The last item in cons list is a value called 'Nil'
//cons list is produced by recursively calling this function.

//When Rust look into normal enum, they can figure out size of the enum
//Regading to Recursive, it will look like Cons(Cons(Cons...infinity))
//To figure out size of data at a compile time, we need to insert indirection (e.g. 'Box', 'Rc' or '&')
//With this List example, we can use Box<T>
//Box<T> is a pointer (smart) and rust always can figure out size of pointer.(because it is just pointing address)
enum List {
    Cons(i32, Box<List>), //two items of cons function
    Nil,                  //non-recursive variant
}

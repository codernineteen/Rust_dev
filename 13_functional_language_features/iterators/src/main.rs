//iterator pattern allows you to perform some task on sequence of items
fn main() {
    //In Rust, iterators is lazy = they don't do anything until you call them
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    //The most common usage of iterators is for loop
    //for loop doesn't need mutable iterators because it takes iterators ownership and makes it mutable in behind scenes.
    for val in v1_iter {
        println!("{}", val);
    }
    //Iterators give you more flexibility to use the same logic with many different kinds of sequences

    //#2
    //Iterator trait only require implementors to define one method: next()
    let mut v1_iter2 = v1.iter(); //we need to make iter mutable to use next method.
    println!("{:?}", v1_iter2.next()); // print result will be Some(value)

    //#3
    //An iterator which takes ownership of origin and returns owned value -> use into_iter() method
    //An iterator which iterated over mutable references, use iter_mut() method

    //#4
    iterator_sum(&v1);
    //#5
    adaptors_map(&v1);
}

//#1
//All the iterators implement trait named 'Iterator'
//Definition is
pub trait Iterator {
    //This 'type item' and 'Self::Item' will be discussed in Chapter 19
    //For now, you only need to know we need both of this syntax for Iterator trait

    type Item;

    fn next(&mut self) -> Option<Self::Item>; //Item type will be the type returned from the iterator

    // methods with default implementations elided
}

//#4
//Methods of iterators
//There are lots of useful methods of iterators from std library.
fn iterator_sum(v: &[u32]) {
    let total: u32 = v.iter().sum(); //sum takes ownership
    println!("{}", total);
}

//#5
//iterator adaptors: change one iterator into different kinds of iterators
//As we said, iterators is lazy , so we should consume it to get results from adaptors
//one example is a map method (map takes a closure)
fn adaptors_map(v: &[u32]) {
    //Because map takse closure, we can do any operation we want to do with iterators.
    let v1: Vec<_> = v.iter().map(|x| x + 1).collect(); //.collect method is a way to consume iterators.
    println!("{:?}", v1);
}

//#6
//using closure that capture their environment
//'filter' iterator adaptor -> filter takes each item -> returns bool
// if true -> the value will be included in iterator / if false, the value won't be included
// go to lib.rs

//#7
// go to lib.rs

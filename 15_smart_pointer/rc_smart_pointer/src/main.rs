//a single value might have multiple owners -> Rc<T>(Reference counting) enable this
//e.g. Tv is a value, and everyone who watches Tv is owners
//If last Tv watchers leave watching Tv, Tv(value) will be cleaned up
//Anyone cannot turn off the Tv while other watching (clean up value while other owners use it)
//Rc<T> is single-threaded scenarios
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    //b and c share ownership
    let b = Cons(3, Rc::clone(&a)); //This is not deep copy. only counting references
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//But Rc<T> can't have multiple mutable references(data races problem) as itself.
//RefCell<T> can't figure out this problem.

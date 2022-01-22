// types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime,
// even though the compiler can’t guarantee that.
// RefCell<T> is also a type that follows interior mutablility

//Borrowing rule :
//1. At any given time, one mutable reference or any number of immutable references
//2. references must always be valid

//Difference between Box<T> and RefCell<T
//Box<T> follows these rules at a compile time. If break rules -> compile error
//RefCell<T> follows these rules at a run time. If break rules -> program panic
//The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

//Refcell<T> in combination with Rc<T>
#[derive(Debug)]
enum List {
    //RefCell<T> to gain the ability to change the values in the list
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10; // * to dereference Rc<T> to the inner Refcell<T>

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

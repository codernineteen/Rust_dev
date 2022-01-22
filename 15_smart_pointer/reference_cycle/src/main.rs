use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    //RefCell modify List value
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // a list that points to the list in a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        //With RefCell, we makes tail points to b instead of Nil.
        //created reference cycle
        //Here Reference count of b goes up til 2
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
    hardcoded_example();
}
//Rust drops b, which decreases count of the Rc<List> instance from 2 to 1
// Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1
//Because count is 1, not 0, The memory won't be dropped entirely

//The cycle is :
// b -> 10 | RefCell -> 5 | RefCell (which a points to)
// a -> 5 | RefCell -> 10 | RefCell (by changing tail of a)
// So last step of a points to b, and last step of b points to b. It makes cycle

//A way to solve this problem is to use Weak<T> smart pointer

//Creating a Tree Data structure
#[derive(Debug)]
struct Node {
    value: i32,
    //With RefCell, we can modify which nodes are children of another node.
    //with Rc, we can share that ownership with variables. -> can access each Node in the tree directly
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn hardcoded_example() {
    //Owner of Node in leaf : leaf(itself) and branch(who share ownership of leaf)
    let leaf = Rc::new(Node {
        value: 3,
        //start without parent
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        //store leaf in branch
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

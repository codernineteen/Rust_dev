//Mutex = mutual exclustion, this allow only one thread to access some data at any given time.
//mutex is described as guarding the data it holds via the locking system.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //create mutex
    let m = Mutex::new(5);
    //To access the data in mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock
    //lock : a data structure which tracks of who has exclusive access
    {
        //acquire lock
        //lock method will block current thread so it can't do any work until its our turn to have the lock
        //call to 'lock' method returns a smart pointer 'MutexGuard'
        let mut num = m.lock().unwrap();
        *num = 6;
    } //unlock when MutexGuard goes out of scope

    println!("m = {:?}", m);
    mutex_multi_threads();
}

//Arc: Atomically reference counted type - an additional kind of concurrency primitive types
//Arc<T> : works similarly to Rc<T>, but is safe to use in concurrent situations.
//The reason we don't use Arc<T> everywhere is because of performanc penalty of Arc<T>
//It is a safe way, but in single thread, it will be better just to use Rc<T>
fn mutex_multi_threads() {
    //To share ownership between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //Mutex<T> provide interior mutability
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    } //first iteration end -> unlock -> second start -> lock -> second end -> unlock and so forth.

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

//Threads : features that run independent parts
use std::thread;
use std::time::Duration;

fn main() {
    //#1
    //To create new thread , thread::spawn and argument is closure.
    //Originally, there is no guarantee that the spawned thread will get to run or whether can be completed or not after running
    //To prevent this, we can assign thread's return to variable
    //thread::spawn return 'JoinHandle' type
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {}", i);
            //this line allow current thread to stop as much as defined time
            //and then another threads running
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("bye number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //#2
    //'move' closure with threads
    //this closure allows you to use data from one thread in another thread
    let v = vec![1, 2, 3];
    //Because rust doens't know when v will be dropped before thread running ended, It will casue compile error if you use v without 'move' keyword
    //'move' keyword takes ownership of value
    let handle_one = thread::spawn(move || {
        //It guarantees that Rust will not use 'v' in main thread anymore
        println!("vector: {:?}", v);
    });

    //This join method waits for asscociated thread to finish. If thread already finished, it return immediately
    //If thread is panic, join return Err
    handle.join().unwrap();
    handle_one.join().unwrap();
}

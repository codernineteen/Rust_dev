//One major tool Rust has for accomplishing message-sending concurrency is the channel
// A channel in programming has two halves : trasmitter and reciever
// transmitter half : upstream location
// receiver half: downstream location

//mpsc: multiple producer, single consumer
//e.g multiple stream -> one big river
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //creating channel -> returns tuple -> (transmitting end, receiver end)
    let (tx, rx) = mpsc::channel();

    //Own transimiiting end to send messages through channel
    thread::spawn(move || {
        let val = String::from("hi");
        //send method return Result<T,E>
        //if receiving end already dropped, there is nowhere to send -> Err
        //takes ownership of val and give receiver its ownership
        tx.send(val).unwrap();
    });

    //recv() -> block main thread's execution and wait until a value is sent down -> return Result type
    //try_recv() -> doesn't block but return Result type immediately (This useful when a thread has other works to do while waiting messages)
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    multiple_producers();
}

fn _send_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //treating rx as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got : {}", received)
    }
}

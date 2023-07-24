use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // when the main thread of a Rust program completes, all spawned threads are shut down,
    // whether or not they have finished running
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // the join here will blocks the main thread,
    // waiting for the spawned thread to finish
    // if it is placed before the main thread's for loop,
    // the spawned thread will finish before the main thread starts
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        // without move, rust infers how to capture v,
        // and because println! only needs a reference to v, the closure tries to borrow v.
        // Rust can’t tell how long the spawned thread will run
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();

    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
            println!("innermost n = {n}");
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("outer n = {n}"); // 2
                               // The move keyword causes n to be copied into the closure,
                               // so the assignments n = n + 1 within thread::spawn have no effect on the outer n.

    // (transmitter, receiver)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("single hi");
        tx.send(val).unwrap(); // send can only send values of a single type
                               // println!("val is {}", val); // error: value borrowed here after move in send function
    });

    let received = rx.recv().unwrap(); // recv blocks the main thread
                                       // try_recv doesn’t block (allows current thread to do other work while waiting)),
                                       // but will instead return the Result<T, E> immediately.

    println!("Got: {}", received);

    // sending multiple values
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

    // iterating over the receiving end of a channel
    // instead of calling recv explicitly, we can treat rx as an iterator
    for received in rx {
        println!("Got: {}", received);
    }

    // multiple transmitters
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
        println!("Got: {}", received);
    }
}

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

    // mutexes
    // “Do not communicate by sharing memory;
    // instead, share memory by communicating.”

    // Mutex Rules
    // - You must attempt to acquire the lock before using the data.
    // - When you’re done with the data that the mutex guards,
    // you must unlock the data so other threads can acquire the lock.

    // single threaded
    use std::sync::Mutex;

    // Mutex<i32>
    // Mutex<T> is a smart pointer called MutexGuard,
    // wrapped in a Lock Result
    // Mutex<T> implements Deref and Drop traits
    // the Drop implemntation releases the lock automatically
    // Mutex provides interior mutability, as Cell and RefCell do

    // todo: create a deadlock

    let m = Mutex::new(5);

    {
        // acquire the lock
        // blocks current thread until it’s able to acquire the lock
        // the call to lock would fail and return an error
        // if another thread holding the lock panicked.

        // num can be treated like a mutable reference
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // mutex in multiple threads
    // requires Arc<T> (atomic reference counting)
    // (not safe to use if its data contains a reference)
    use std::sync::Arc;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // send and sync traits

    // Send allows ownership of values to be transferred between threads

    // Almost any type in Rust is Send
    // Primitive types are Send
    // Any type composed entirely of Send types is automatically marked as Send as well

    // Rc<T> does not implement Send
    // because two threads might try to mutate the reference count at the same time

    // Raw pointers are also not Send

    // RefCell is Send if the type it wraps is Send

    // Mutex<T> is Send

    // MutexGuard<'a, T>, is not Send

    // Sync allows multiple threads to have references to the same data
    // so if T implements Sync, &T is Send
    // primitive types are Sync
    // types composed entirely of types that are Sync are also Sync

    // Rc<T> is not Sync

    // RefCell<T> is not Sync

    // Mutex<T> is Sync

    // MutexGuard<'a, T>, returned by Mutex::lock is Sync
}

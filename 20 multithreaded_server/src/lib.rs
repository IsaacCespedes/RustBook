pub mod hello {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    impl ThreadPool {
        // (note the doc comments below: `cargo doc --open`)

        /// Create a new ThreadPool.
        ///
        /// The size is the number of threads in the pool.
        ///
        /// # Panics
        ///
        /// The `new` function will panic if the size is zero.
        pub fn new(size: usize) -> ThreadPool {
            // returning a Result<ThreadPool, PoolCreationError> is also possible
            assert!(size > 0);

            // the thread pool holds the sender, which acts as the queue for Jobs
            // the vector of workers hold receivers, which are used to receive Jobs
            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        // the idea is for this to work similarly to the standard library’s thread::spawn function
        // spawn uses FnOnce as the trait bound on F
        // also, the request will only be processed once
        // the Send  is implemented to transfer the closure from one thread to another
        // the 'static lifetime is the longest possible lifetime
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            // must drop sender first so that workers know to stop
            drop(self.sender.take());

            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);

                // we need to move the thread out of the Worker instance that owns thread
                // so join can consume the thread
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            // if the OS can't create a thread the whole program will panic
            // for simplicity, we're not handling this case
            // std::thread::Builder::spawn is an alternative that returns a Result
            let thread = thread::spawn(move || loop {
                // lock() can fail if the mutex is poisoned
                // this can happen if a thread panics while holding the lock
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
}

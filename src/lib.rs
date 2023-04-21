use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// ThreadPool implementation
impl ThreadPool {
    /// Create new Thread Pool
    ///
    /// Size is the number of the pool
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        /* Create new channel */
        let (sender, reciver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(reciver));

        /* Create workers for the thread pool */
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    ///Execute a thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            /* no need to unlock because smart pointer */
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Exceuting jobs {}", id);
            job();
        });

        Worker { id, thread }
    }
}

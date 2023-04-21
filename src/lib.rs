use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

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
            workers.push(Worker::new(id, reciver));
        }
        ThreadPool { workers, sender }
    }

    ///Execute a thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<Receiver<Job>>,
}

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| receiver);
        Worker { id, thread }
    }
}

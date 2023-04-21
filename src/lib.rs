use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/* Impl drop for auto drop when out of scope */

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate);
        }

        for worker in &mut self.workers {
            match worker.thread.take() {
                Some(x) => {
                    x.join().unwrap();
                }
                None => {}
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            /* no need to unlock because smart pointer */
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Exceuting worker {}", id);
                    job();
                }
                Message::Terminate => {
                    println!("Terminate worker {}", id);
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

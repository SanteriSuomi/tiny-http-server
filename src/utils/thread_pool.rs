use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

// A thread pool that executes jobs in parallel threads.
pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut _workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            _workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { _workers, sender }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

impl Worker {
    fn new(_id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            _id,
            _thread: thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {_id} got a job; executing.");
                job();
            }),
        }
    }
}

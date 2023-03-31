use std::error::Error;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    size: usize,
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            threads[i] = Worker::new(i, Arc::clone(&receiver));
        }
        ThreadPool {
            size,
            threads,
            sender,
        }
    }

    pub fn execute<F>(&mut self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f))?;
        Ok(())
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                receiver.lock().unwrap().recv().unwrap()();
            }),
        }
    }
}

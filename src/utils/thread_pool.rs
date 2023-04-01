use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

// A thread pool that executes jobs in parallel threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take().unwrap());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

// A worker thread that executes jobs.
struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(_id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            _id,
            thread: Some(thread::spawn(move || loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Worker {_id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {_id} got a shutdown message.");
                        break;
                    }
                }
            })),
        }
    }
}

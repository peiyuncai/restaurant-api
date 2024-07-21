use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use crate::libraries::job::Job;
use crate::libraries::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            let worker = Worker::new(id, receiver);
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap()
    }

    pub fn join(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use crate::libraries::job::Job;
use crate::libraries::worker::Worker;

pub trait ThreadPoolDyn: Send + Sync {
    fn execute(&self, job: Box<dyn FnOnce() + Send + 'static>);
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
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

        ThreadPool { workers, sender: Some(sender) }
    }
}

impl ThreadPoolDyn for ThreadPool {
    fn execute(&self, job: Box<dyn FnOnce() + Send + 'static>) {
        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

/*
self.sender.take()
Purpose: The take() method is used to replace the value of self.sender with None and return the original value.

Why take()?:
self.sender is an Option<mpsc::Sender<Job>>. By calling take(), we consume the Option and retrieve the Sender, leaving self.sender as None.
This ensures that no more jobs can be sent to the channel because the sender is dropped, effectively closing the channel.

Mutable Self: We need &mut self because take() modifies self.sender by replacing it with None.
Dropping self.sender

Why Drop?: By explicitly dropping the sender, we signal to all the worker threads that no more jobs will be sent. The worker threads will eventually encounter an error when trying to receive a job from the closed channel and will then exit their loop.

Iterating Over Workers
&mut self.workers: This mutable reference allows us to modify each worker's thread field.
Taking the Thread: worker.thread.take() replaces the thread field with None and returns the original JoinHandle.
Joining the Thread: Calling thread.join().unwrap() ensures that the main thread waits for the worker thread to finish execution before continuing. This is essential for clean shutdown and ensures that all worker threads have completed their tasks.
 */

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

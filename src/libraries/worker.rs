use std::error::Error;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::libraries::job::Job;

pub struct Worker {
    id: usize,
    pub(crate) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let result = receiver.lock().unwrap().recv();
                match result {
                    Ok(job) => {
                        println!("Worker {id} starts executing the job");
                        job();
                        println!("Worker {id} finished the job");
                    }
                    Err(err) => {
                        println!("Worker {id} encountered an error: {:?}", err.source());
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
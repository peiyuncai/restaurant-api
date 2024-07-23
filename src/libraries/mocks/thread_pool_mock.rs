use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use crate::libraries::thread_pool::ThreadPoolDyn;

pub struct MockThreadPool {
    counter: Arc<AtomicUsize>,
    threads: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
}

impl MockThreadPool {
    pub fn new() -> Self {
        MockThreadPool {
            counter: Arc::new(AtomicUsize::new(0)),
            threads: Arc::new(Mutex::new(Vec::new())), //plus main thread
        }
    }

    pub fn get_count(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn wait(&self) {
        let mut threads = self.threads.lock().unwrap();
        for handle in threads.drain(..) {
            handle.join().unwrap();
        }
    }
}

impl ThreadPoolDyn for MockThreadPool {
    fn execute(&self, _: Box<dyn FnOnce() + Send + 'static>) {
        let counter = self.counter.clone();
        let mut threads = self.threads.lock().unwrap();
        let handle = thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
        });
        threads.push(handle);
    }
}
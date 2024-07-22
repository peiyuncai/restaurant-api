use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use tokio::sync::Barrier;
use crate::libraries::thread_pool::ThreadPoolDyn;

pub struct MockThreadPool {
    counter: Arc<AtomicUsize>,
    barrier: Arc<Barrier>,
}

impl MockThreadPool {
    pub fn new(size: usize) -> Self {
        MockThreadPool {
            counter: Arc::new(AtomicUsize::new(0)),
            barrier: Arc::new(Barrier::new(size)),
        }
    }

    pub fn get_count(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn wait(&self) {
        _ = self.barrier.wait();
    }
}

impl ThreadPoolDyn for MockThreadPool {
    fn execute(&self, _: Box<dyn FnOnce() + Send + 'static>) {
        let counter = self.counter.clone();
        let barrier = self.barrier.clone();
        thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
            _ = barrier.wait();
        });
    }
}
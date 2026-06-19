use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

pub struct ThreadHandle {
    pub sender: Sender<Box<dyn FnOnce() + Send>>,
}

impl ThreadHandle {
    pub fn new() -> Self {
        let (tx, rx): (Sender<Box<dyn FnOnce() + Send>>, Receiver<_>) = channel();

        thread::spawn(move || {
            for job in rx {
                job(); 
            }
        });

        Self { sender: tx }
    }

    pub fn run_on_thread<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.send(Box::new(f));
    }
}

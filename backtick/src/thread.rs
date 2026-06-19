use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::{future::Future, pin::Pin};
use once_cell::sync::Lazy;

pub struct ThreadHandle {
    pub sender: Sender<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>>,
}

pub static THREAD: Lazy<ThreadHandle> = Lazy::new(|| ThreadHandle::new());


impl ThreadHandle {
    pub fn new() -> Self {
        // Explicit type annotation REQUIRED
        let (tx, rx): (
            Sender<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>>,
            Receiver<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>>,
        ) = channel();

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            loop {
                match rx.recv() {
                    Ok(job) => rt.block_on(job()),
                    Err(_) => {
                        // Sender dropped — keep thread alive
                        thread::park();
                    }
                }
            }
        });

        Self { sender: tx }
    }

    pub fn run_on_thread<F, Fut>(&self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let _ = self.sender.send(Box::new(move || Box::pin(f())));
    }
}

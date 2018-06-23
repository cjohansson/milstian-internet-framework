use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

pub struct Pool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Pool {

    /// Create a job from a closure and send for execution
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
    {
        // Place job inside a Box
        let job = Box::new(f);

        // Send a NewJob Message down the channel
        // TODO Handle this unwrap
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    /// Create a new mutex channel with specified number of receivers
    pub fn new (size: usize) -> Pool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver)
            ));
        }

        Pool {
            workers,
            sender
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // Identical number of terminate messages and workers assure
        // all workers will receive the message
        for _ in &mut self.workers {
            // TODO Handle this unwrap
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // Join every workers thread
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                // TODO handle this unwrap
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // TODO Handle thes unwraps
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

type Job = Box<FnBox + Send + 'static>;

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

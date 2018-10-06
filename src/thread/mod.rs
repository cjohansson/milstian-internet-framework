//! # Handles the workers

// TODO Add tests to this file

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use Application;

pub struct Pool<'a> {
    application: &'a Application,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl<'a> Pool<'a> {
    /// Create a job from a closure and send for execution
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Place job inside a Box
        let job = Box::new(f);

        // Send a NewJob Message down the channel
        // If it fails program will crash deliberately
        self.sender
            .send(Message::NewJob(job))
            .expect("Failed to send job down to channel");
    }

    /// Create a new mutex channel with specified number of receivers
    pub fn new(application: &'a Application, size: usize) -> Pool {
        assert!(size > 0);
        application
            .get_feedback()
            .info(format!("Starting {} new workers", size));

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(&application, id, Arc::clone(&receiver)));
        }

        Pool {
            application,
            sender,
            workers,
        }
    }
}

impl<'a> Drop for Pool<'a> {
    fn drop(&mut self) {
        self.application
            .get_feedback()
            .info("Sending terminate message to all workers.".to_string());

        // Identical number of terminate messages and workers assure
        // all workers will receive the message
        for _ in &mut self.workers {
            if let Err(error) = self.sender.send(Message::Terminate) {
                self.application.get_feedback().error(format!(
                    "Failed to send a termination message, error: {:?}",
                    error
                ));
            }
        }

        self.application
            .get_feedback()
            .info("Shutting down all workers.".to_string());

        // Join every workers thread
        for worker in &mut self.workers {
            self.application
                .get_feedback()
                .info(format!("Shutting down worker {}", worker.id));

            if let Some(thread) = worker.thread.take() {
                if let Err(error) = thread.join() {
                    self.application.get_feedback().info(format!(
                        "Failed to join thread {:?}, error: {:?}",
                        worker.thread, error
                    ));
                }
            } else {
                self.application.get_feedback().info(format!(
                    "Failed to take ownership of thread {:?}",
                    worker.thread
                ));
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl<'a> Worker {
    fn new(
        application: &'a Application,
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    ) -> Worker {
        let application_clone = application.clone();

        let thread = thread::spawn(move || loop {
            if let Ok(lock) = receiver.lock() {
                if let Ok(message) = lock.recv() {
                    match message {
                        Message::NewJob(job) => {
                            application_clone
                                .get_feedback()
                                .info(format!("Worker {} started executing", id));
                            job.call_box();
                            application_clone
                                .get_feedback()
                                .info(format!("Worker {} finished executing", id));
                        }
                        Message::Terminate => {
                            application_clone
                                .get_feedback()
                                .info(format!("Worker {} was told to terminate", id));
                            break;
                        }
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

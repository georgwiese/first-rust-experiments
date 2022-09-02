use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            match job {
                Message::NewJob(job) => {
                    println!("Worker {} handling job!", { id });
                    // TODO: The book here did (*job)(), which then had to be wrapped in a trait for
                    // dubious reasons. But this seems to work just fine?
                    job();
                    println!("Worker {} finished job.", { id });
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", { id });
                    break;
                }
            }
        });
        Worker { id, handle }
    }
}

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic of the size is zero.

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        // TODO: Is this really the way to go?
        // That's the way it's done in the book, but it seems strange to wrap a channel
        // with the mutex?
        // AFAIK the only feature of channels we really use here is the behavior that
        // it blocks until there is a new element. We could as well use a queue + polling.
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Telling all workers to terminate");
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");
        // TODO: In the book, they don't use pop() and instead get a mutable reference,
        // and then use Option.take() to get ownership. This seems like a cleaner solution.
        while let Some(worker) = self.workers.pop() {
            println!("Shutting down worker {}", worker.id);
            worker.handle.join().unwrap()
        }
    }
}

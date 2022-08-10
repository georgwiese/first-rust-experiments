use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn book_example() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

static PARAMS: [u64; 3] = [27, 42, 97];
static PRIME: u64 = 479001599;
static EXPONENT: u64 = 1e8 as u64;

fn exp(value: u64, exponent: u64, prime: u64) -> u64 {
    let mut result: u64 = 1;
    for _ in 0..exponent {
        result = result * value % prime;
    }
    result
}

fn message_passing_test() {
    let (tx, rx) = mpsc::channel();

    println!("Starting computation");

    PARAMS.map(|param| {
        let tx = tx.clone();
        thread::spawn(move || tx.send((param, exp(param, EXPONENT, PRIME))).unwrap());
    });

    // This ensures that the channel is closed once the last thread is finished.
    // `tx` is never actually used, except for cloning it once for each worker thread.
    // If we wouldn't drop it here, the loop below would never terminate.
    drop(tx);

    for (param, result) in rx {
        println!(
            "Result for {}^{} (mod {}) = {}",
            param, EXPONENT, PRIME, result
        );
    }
}

fn mutex_test() {
    let results: Arc<Mutex<HashMap<u64, u64>>> = Arc::new(Mutex::new(HashMap::new()));

    println!("Starting computation");

    let handles = PARAMS.map(|param| {
        let worker_results = results.clone();
        thread::spawn(move || {
            let result = exp(param, EXPONENT, PRIME);

            let mut results_pointer = worker_results.lock().unwrap();
            results_pointer.insert(param, result);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    for (param, result) in results.lock().unwrap().iter() {
        println!(
            "Result for {}^{} (mod {}) = {}",
            param, EXPONENT, PRIME, result
        );
    }
}

pub fn main() {
    book_example();
    message_passing_test();
    mutex_test();
}

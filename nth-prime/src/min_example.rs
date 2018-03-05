use std::thread;
use std::sync::{Arc, Mutex};

pub fn main() {
    let size = 100;
    let threads = 3;
    let vector = Arc::new(Mutex::new(vec![0; size]));
    let mut handles = vec![];

    for _ in 0..threads {
        let vector = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

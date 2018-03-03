use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let msgs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(msg * 30));
        }
    });

    thread::spawn(move || {
        let msgs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(msg * 20));
        }
    });

    for received in rx {
        println!("MAIN received {0}", received);
    }
}

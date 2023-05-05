use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data)
    });
    let _ = tx.send("Hello world");
    let _ = handle.join();
    // let mut handles = Vec::new();
    // let mut data = vec![1; 10];
    // let mut snd_channel = Vec::new();
}

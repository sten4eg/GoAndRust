use std::sync::Arc;
use std::sync::atomic::{AtomicI32};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::atomic::Ordering::Relaxed;


fn main() {
    let counter = Arc::new(AtomicI32::new(0));
    let (tx, rx) = channel();

    for _ in 0..20 {
        let (counter, tx) = (counter.clone(), tx.clone());
        thread::spawn(move || {
            for _ in 0..1000000 {
                counter.fetch_add(1, Relaxed);
            }
            tx.send(());
        });
    }

    for _ in 0..20 { rx.recv(); }
    println!("{}", counter.load(Relaxed));
}
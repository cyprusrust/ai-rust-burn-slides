use std::thread;
use std::time::Duration;
use std::sync::Arc;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let data1 = data.clone();
    let data2 = data.clone();

    let handle1 = thread::spawn(move || {
        for i in data1.iter() {
            println!("Thread 1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in data2.iter() {
            println!("Thread 2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

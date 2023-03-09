use std::thread;
use std::time::Duration;

fn main() {
    //FIX ME 

    let data = vec![1, 2, 3];

    let handle1 = thread::spawn(move || {
        for i in data.iter() {
            println!("Thread 1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in data.iter() {
            println!("Thread 2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

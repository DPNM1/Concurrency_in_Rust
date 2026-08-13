use std::sync::{Arc, RwLock};
use std::thread;
fn main() {
    let counter = Arc::new(RwLock::new(1));
    let count1 = counter.clone();
    let count2 = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 1..10 {
            let mut writer = count1.write().unwrap();
            *writer += 1;
        }
    });
    let handle2 = thread::spawn(move || {
        let reader1 = count2.read().unwrap();
        println!("From Reader 1 , Counter = {}", *reader1);
    });
    let handle3 = thread::spawn(move || {
        let reader2 = counter.read().unwrap();
        println!("From Readrer 2 , Counter = {}", *reader2);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}

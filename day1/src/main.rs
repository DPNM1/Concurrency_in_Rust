use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let counter = Arc::new(Mutex::new(1));
    let count1 = Arc::clone(&counter);
    let count2 = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        for _ in 1..1000 {
            let mut num = count1.lock().unwrap();
            *num += 1;
        }
    });
    let handle2 = thread::spawn(move || {
        for _ in 1..1000 {
            let mut num2 = count2.lock().unwrap();
            *num2 += 1;
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The final counter value is {}", *counter.lock().unwrap());
}

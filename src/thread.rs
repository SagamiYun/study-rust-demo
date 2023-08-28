use std::sync::{Arc, Barrier};
use std::thread;

fn thread_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(1));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move|| {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


pub fn thread_main() {

    // 线程屏障
    thread_barrier();
}
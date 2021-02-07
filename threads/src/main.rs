use std::thread;
use std::time::Duration;

fn main() {
    println!("-------------------- Run many threads example:");
    let thread_handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread_handler.join().unwrap(); // before this thread wont be finished code under will not execute
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("-------------------- Pass data from main thread to spawned thread:");
    let main_thread_vector = vec![1, 2, 3];

    // This will throw an error because we are trying to access a variable which hase ownership in other scope
    // The error will occur because parent scope can drop this variable so then when spawned thread will try to get value under this variable it will access null because variables was dropped by main scope
    // let handle = thread::spawn(|| {
    //     println!("Variable from main thread: {:?}", main_thread_vector);
    // });

    // Here we are forcing to move the ownership of main thread variable to spawned thread scope
    let handle = thread::spawn(move || {
        println!("Variable from main thread: {:?}", main_thread_vector);
    });

    // tThis will throw an error because we moved main thread variable to spawned thread scope so we can not access this data from main scope anymore
    // println!("{:?}", main_thread_vector);
    handle.join().unwrap();
}

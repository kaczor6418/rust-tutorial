use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
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

    println!("\n");
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

    println!("\n");
    println!("---------------------------------- Message passing (Streams):");
    let (transmitter, receiver) = mpsc::channel();
    thread::spawn(move || {
        let messages_to_send = vec![
            String::from("first message"),
            String::from("second message"),
            String::from("third message"),
        ];
        for message in messages_to_send {
            transmitter.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // To receive single message we ca use .recv but this method will block main thread :/
    // let received_message = receiver.recv().unwrap();

    for received in receiver {
        println!("Received message: {}", received);
    }
    println!("Main thread is not blocked");

    println!("\n");
    println!("---------------------------------- Multiple transmitters:");
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let values = vec![
            String::from("# FIRST THREAD: 1"),
            String::from("# FIRST THREAD: 2"),
            String::from("# FIRST THREAD: 3"),
            String::from("# FIRST THREAD: 4"),
        ];
        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vales = vec![
            String::from("@ SECOND THREAD: 1"),
            String::from("@ SECOND THREAD: 2"),
            String::from("@ SECOND THREAD: 3"),
            String::from("@ SECOND THREAD: 4"),
        ];
        for val in vales {
            tx.send(val).unwrap();
        }
    });

    let message_listener = thread::spawn(move || {
        for received in rx {
            println!("Received message: {}", received);
        }
    });

    let main_thread_messages = vec![
        String::from("& MAIN THREAD: 1"),
        String::from("& MAIN THREAD: 2"),
        String::from("& MAIN THREAD: 3"),
        String::from("& MAIN THREAD: 4"),
    ];
    for message in main_thread_messages {
        println!("MAIN THREAD MESSAGE: {}", message);
    }
    message_listener.join().unwrap();

    println!("\n");
    println!("---------------------------------- Shared state (Mutex):");
    let shared_value = Mutex::new(5);
    {
        let mut num = shared_value.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", shared_value);

    println!("\n");
    println!("---------------------------------- Shared state in multiple threads:");
    let counter = Arc::new(Mutex::new(0));
    let mut threads_handlers = vec![];

    for i in 0..10 {
        let counter_pointer = Arc::clone(&counter);
        let thread_handler = thread::spawn(move || {
            println!("THREAD NO: {}", i);
            let mut mutable_counter = counter_pointer.lock().unwrap();
            *mutable_counter += 1;
        });
        threads_handlers.push(thread_handler);
    }

    for thread_handler in threads_handlers {
        thread_handler.join().unwrap();
    }

    println!(
        "After all threads finish their work: {}",
        *counter.lock().unwrap()
    );
}

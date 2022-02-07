use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if handle() is called here then it'll let the spawned thread finish before starting the main thread

    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1))
    }
    // this allows spawn thread to finish.
    handle.join().unwrap();

    /*======================================================================== */

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // data we want to transmit
    thread::spawn(move || {
        let msg = vec![
            String::from("hello"),
            String::from("hi"),
            String::from("hey"),
            String::from("yea"),
            String::from("we"),
        ];

        for message in msg {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // separate data we want to transmit
    thread::spawn(move || {
        let msg = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
            String::from("we"),
        ];

        for message in msg {
            tx2.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // data we received
    for received in rx {
        println!("got: {}", received);
    }

    /*======================================================================== */

    //Creates a new mutex in an unlocked state ready for use
    let m = Mutex::new(5);

    {
        // lock() returns a result type, if another threads has the data then it will fail
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m  = {:?}", m);

    // A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted'
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Returns a copy of the value.
        let counter = Arc::clone(&counter);

        // Moves counter into the thread, which takes ownership.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("results: {}", *counter.lock().unwrap());
}

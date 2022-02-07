# Concurrency

## Creating Threads

They're two types of threads:

1. 1 to 1 threads or system threads
   * When you create a thread in your program it maps to an OS thread.
2. Green threads, user threads, program threads
   * 20 green threads can map to 4 OS threads.

> Rust mainly uses 1 to 1 threads.

One thing to note when using threads, is that when the main thread is complete the spawn threads are complete. Unless specified using a `join()`. This will return a result so we will need to use unwrap().

When putting the `move` keyword in front of a closure, we are telling rust don't infer and take ownership of the outside keyword. We have to do this cause Rust does not know that when this thread could end.

```rs
let v1 = vec![1, 2, 3]

let handle = thread::spawn(move || {
      for i in 1..10 {
          println!("number {} from the spawned thread", i);
          thread::sleep(Duration::from_millis(1));
      }
  });
```

***

## Message passing

Message passing - is when you have threads passing data to each other. Rust uses a `channel` to pass messages. It has two halves the **transmitter** and the **receiver**.

***

## Sharing State

Shared State - is one way data flow, transferring ownership between threads.

mutex - mutual exclusion, some piece of data and only one thread can access it. One thread will signal that it wants to acquire a locke on that data.

Two rules:

1. Acquire a lock to get access to the data
2. Release the data when you are finished

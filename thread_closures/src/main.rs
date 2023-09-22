// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter()
        .filter(|&x| { x % 2 == 0 })
        .map(|&x| { x * x })
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    to_channel();
}

fn to_channel() {
    let (tx, rx) = channel::unbounded();

    let rx2 = rx.clone();

    let handle_a = thread::spawn(move || {
        while let Ok(val) = rx.recv() {
            pause_ms(1);
            println!("Thread 1 received: {}", val);
        }
    });

    pause_ms(100); 

    let handle_b = thread::spawn(move || {
        while let Ok(val) = rx2.recv() {
            pause_ms(1);
            println!("Thread 2 received: {}", val);
        }
    });

    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        let _ = tx.send(letter);
    }

    drop(tx);

    handle_a.join().unwrap();
    handle_b.join().unwrap();
    println!("Main thread: Exiting.")
}

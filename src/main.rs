use std::sync::mpsc;
use std::thread;

mod point;
mod tally;

use point::Point;
use tally::Tally;

fn sample(times: usize) -> Tally {
    let mut tally = Tally::new();

    for _ in 0..times {
        let hit = Point::new().within_unit_circle();
        tally.count(hit);
    }

    tally
}

fn main() {
    let (tx,rx) = mpsc::channel();

    let thread_count = match thread::available_parallelism() {
        Ok(available) => available.get(),
        Err(_) => 1
    };

    println!("Sampling across {} thread(s)...", thread_count);

    for _ in 0..thread_count {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let thread_tally = sample(1000000);
            tx.send(thread_tally).unwrap();
        });
    }
    drop(tx);

    let mut tally = Tally::new();
    for incoming_tally in rx {
        tally += incoming_tally;
    }

    println!("Result: {}", tally);
}

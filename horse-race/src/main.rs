use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::prelude::*;

fn main() {
    // println!("Hello, world!");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();

    thread::spawn(move ||{
        let mut rng = rand::thread_rng();
        let mut y: f64 = 0f64;
        while y < 100f64 {
            let rand: f64 = rng.gen();
            y += rand * 10f64;
        }
        tx.send(String::from(format!("Horse 1: {}", y))).unwrap();
    });
    
    thread::spawn(move ||{
        let mut rng = rand::thread_rng();
        let mut y: f64 = 0f64;
        while y < 100f64 {
            let rand: f64 = rng.gen();
            y += rand * 10f64;
        }
        tx1.send(String::from(format!("Horse 2: {}", y))).unwrap();
    });

    thread::spawn(move ||{
        let mut rng = rand::thread_rng();
        let mut y: f64 = 0f64;
        while y < 100f64 {
            let rand: f64 = rng.gen();
            y += rand * 10f64;
        }
        tx2.send(String::from(format!("Horse 3: {}", y))).unwrap();
    });

    thread::spawn(move ||{
        let mut rng = rand::thread_rng();
        let mut y: f64 = 0f64;
        while y < 100f64 {
            let rand: f64 = rng.gen();
            y += rand * 10f64;
        }
        tx3.send(String::from(format!("Horse 4: {}", y))).unwrap();
    });

    thread::spawn(move ||{
        let mut rng = rand::thread_rng();
        let mut y: f64 = 0f64;
        while y < 100f64 {
            let rand: f64 = rng.gen();
            y += rand * 10f64;
        }
        tx4.send(String::from(format!("Horse 5: {}", y))).unwrap();
    });

    let rec = rx.recv().unwrap();
    println!("{}", rec);
}

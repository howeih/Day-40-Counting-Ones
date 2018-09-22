extern crate rand;

use rand::prelude::*;
use std::collections::HashMap;
use std::i32;
use std::iter::Iterator;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

enum Data {
    Code(bool),
    Estimate(i32, i32),
}

fn stream_counter(rx: Receiver<Data>) -> thread::JoinHandle<()> {
    let mut bucket = HashMap::<i32, Vec<i32>>::new();
    bucket.insert(1, Vec::<i32>::new());
    let mut timestamp = 1i32;
    let z = thread::spawn(move || loop {
        match rx.recv() {
            Ok(v) => match v {
                Data::Code(code) => {
                    if code {
                        bucket.get_mut(&1).unwrap().push(timestamp);
                        let mut key = 1i32;
                        while bucket.get(&key).unwrap().len() == 3 {
                            if !bucket.contains_key(&key) {
                                bucket.insert(key, Vec::<i32>::new());
                            }
                            let next_key = key * 2;
                            if !bucket.contains_key(&next_key) {
                                bucket.insert(next_key, Vec::<i32>::new());
                            }
                            let val = bucket.get(&key).unwrap()[1];
                            bucket.get_mut(&next_key).unwrap().push(val);
                            bucket.get_mut(&key).unwrap().remove(0);
                            bucket.get_mut(&key).unwrap().remove(0);
                            key = next_key;
                        }
                    }
                    timestamp += 1;
                }
                Data::Estimate(code, n) => {
                    let mut counts = Vec::<i32>::new();
                    for i in bucket.keys() {
                        for t in bucket.get(i).unwrap() {
                            if code < *t {
                                counts.push(*i);
                            }
                        }
                    }
                    let mut estimate: i32 = counts.iter().sum();
                    let mut c: i32 = 0;
                    match counts.iter().last() {
                        Some(s) => {
                            c = s / 2;
                        }
                        None => {}
                    }
                    estimate -= c;
                    println!("last {} bits: {}", n - code, estimate);
                }
            },
            Err(_) => break,
        }
    });
    z
}

fn main() {
    let mut rng = thread_rng();
    let n = 10i32.pow(5);
    let join_handler = {
        let (tx, rx): (Sender<Data>, Receiver<Data>) = mpsc::channel();
        let join_handler = stream_counter(rx);
        for _ in 0..n {
            let x: f64 = rng.gen();
            tx.send(Data::Code(x >= 0.5)).ok();
        }
        let start = 0.99;
        for i in 0..5 {
            let k = n as f64 * (start - i as f64 * 0.99 / 4.);
            tx.send(Data::Estimate(k as i32, n)).ok();
        }
        join_handler
    };
    join_handler.join().ok();
}

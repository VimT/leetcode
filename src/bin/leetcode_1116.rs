//! 打印零与奇偶数

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;

struct ZeroEvenOdd {
    state: AtomicU8,
    control: AtomicBool,
    target: u16,
}

impl ZeroEvenOdd {
    fn new(n: u16) -> Self {
        ZeroEvenOdd { state: AtomicU8::new(0), control: AtomicBool::new(false), target: n }
    }

    fn zero(&self) {
        for _ in 0..self.target {
            while self.state.load(Ordering::Relaxed) != 0 {
                thread::yield_now();
            }
            print!("0");
            if self.control.load(Ordering::Relaxed) {
                self.state.store(2, Ordering::Release);
            } else {
                self.state.store(1, Ordering::Release);
            }
        }
    }

    fn odd(&self) {
        for i in (1..=self.target).step_by(2) {
            while self.state.load(Ordering::Relaxed) != 1 {
                thread::yield_now();
            }
            print!("{}", i);
            self.control.store(true, Ordering::Release);
            self.state.store(0, Ordering::Release);
        }
    }

    fn even(&self) {
        for i in (2..=self.target).step_by(2) {
            while self.state.load(Ordering::Relaxed) != 2 {
                thread::yield_now();
            }
            print!("{}", i);
            self.control.store(false, Ordering::Release);
            self.state.store(0, Ordering::Release);
        }
    }
}

fn main() {
    let zeo = Arc::new(ZeroEvenOdd::new(7));
    let z1 = zeo.clone();
    let t1 = thread::spawn(move || z1.zero());
    let z2 = zeo.clone();
    let t2 = thread::spawn(move || z2.odd());
    let z3 = zeo.clone();
    let t3 = thread::spawn(move || z3.even());
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
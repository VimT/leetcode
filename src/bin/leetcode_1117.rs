//! H2O 生成

use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;

struct H2O {
    h: AtomicU8,
}

impl H2O {
    fn new() -> Self {
        H2O { h: AtomicU8::new(0) }
    }

    fn hydrogen(&self) {
        while self.h.load(Ordering::Relaxed) == 2 {
            thread::yield_now();
        }
        print!("H");
        self.h.fetch_add(1, Ordering::Release);
    }

    fn oxygen(&self) {
        while self.h.load(Ordering::Relaxed) != 2 {
            thread::yield_now();
        }
        print!("O");
        self.h.store(0, Ordering::Release);
    }
}

/// 从leetcode的c++执行结果来看，CondVar更快一些
struct H2OCond {
    lock: Mutex<u8>,
    cond: Condvar,
}

impl H2OCond {
    fn new() -> Self {
        H2OCond { lock: Mutex::new(0), cond: Condvar::new() }
    }

    fn hydrogen(&self) {
        let mut h = self.lock.lock().unwrap();
        h = self.cond.wait_while(h, |x| *x == 2).unwrap();
        print!("H");
        *h += 1;
        self.cond.notify_one();
    }

    fn oxygen(&self) {
        let mut h = self.lock.lock().unwrap();
        h = self.cond.wait_while(h, |x| *x != 2).unwrap();
        print!("O");
        *h = 0;
        self.cond.notify_one();
    }
}


fn main() {
    let h2o = Arc::new(H2O::new());
    let z1 = h2o.clone();
    let t1 = thread::spawn(move || for _ in 0..16 { z1.hydrogen() });
    let z2 = h2o.clone();
    let t2 = thread::spawn(move || for _ in 0..8 { z2.oxygen() });
    t1.join().unwrap();
    t2.join().unwrap();
    println!();

    let h2o = Arc::new(H2OCond::new());
    let z1 = h2o.clone();
    let t1 = thread::spawn(move || for _ in 0..16 { z1.hydrogen() });
    let z2 = h2o.clone();
    let t2 = thread::spawn(move || for _ in 0..8 { z2.oxygen() });
    t1.join().unwrap();
    t2.join().unwrap();
}
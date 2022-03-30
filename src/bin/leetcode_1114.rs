//! 按序打印

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

struct Foo {
    state: AtomicU8,
}

impl Foo {
    fn new() -> Self {
        Foo { state: AtomicU8::new(1) }
    }

    fn first(&self) {
        while self.state.load(Ordering::Relaxed) != 1 {
            thread::yield_now();
        }
        println!("first");
        self.state.store(2, Ordering::Release);
    }

    fn second(&self) {
        while self.state.load(Ordering::Relaxed) != 2 {
            thread::yield_now();
        }
        println!("second");
        self.state.store(3, Ordering::Release);
    }

    fn third(&self) {
        while self.state.load(Ordering::Relaxed) != 3 {
            thread::yield_now();
        }
        println!("third");
        self.state.store(1, Ordering::Release);
    }
}


fn main() {
    let foo = Arc::new(Foo::new());
    let f3 = foo.clone();
    let t3 = thread::spawn(move || { f3.third(); });
    let f1 = foo.clone();
    let t1 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        f1.first();
    });
    let f2 = foo.clone();
    let t2 = thread::spawn(move || { f2.second(); });
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
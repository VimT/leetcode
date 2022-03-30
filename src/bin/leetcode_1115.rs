//! 交替打印FooBar

use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

// 原子操作
struct FooBarAtomic {
    n: i32,
    foo_done: AtomicBool,
}

impl FooBarAtomic {
    fn new(n: i32) -> Self {
        Self { n, foo_done: AtomicBool::new(false) }
    }
    fn foo(&self) {
        for _ in 0..self.n {
            while self.foo_done.load(Ordering::Acquire) {
                thread::yield_now();
            }
            print!("foo");
            self.foo_done.store(true, Ordering::Release);
        }
    }

    fn bar(&self) {
        for _ in 0..self.n {
            while !self.foo_done.load(Ordering::Acquire) {
                thread::yield_now();
            }
            print!("bar");
            self.foo_done.store(false, Ordering::Release);
        }
    }
}

// ------------------------------------
/// 互斥锁+条件变量
struct FooBarMutex {
    n: i32,
    mtx: Mutex<bool>,
    cv: Condvar,
}

impl FooBarMutex {
    fn new(n: i32) -> Self {
        Self { n, mtx: Mutex::new(false), cv: Condvar::new() }
    }
    fn foo(&self) {
        for _ in 0..self.n {
            let mut foo_done = self.mtx.lock().unwrap();
            // 条件变量：如果不满足条件，会释放这个锁，直到满足条件
            foo_done = self.cv.wait_while(foo_done, |x| *x).unwrap();
            print!("foo");
            *foo_done = true;
            self.cv.notify_one();
        }
    }

    fn bar(&self) {
        for _ in 0..self.n {
            let mut foo_done = self.mtx.lock().unwrap();
            foo_done = self.cv.wait_while(foo_done, |x| !*x).unwrap();
            print!("bar");
            *foo_done = false;
            self.cv.notify_one();
        }
    }
}

// ---------------------
/// 信号量
/// Semaphore foo = new Semaphore(1);
/// Semaphore bar = new Semaphore(0);
/// foo:
///     foo.acquire();
///     print!("foo");
///     bar.release();
/// bar:
///     bar.acquire();
///     print!("bar");
///     foo.release();

fn main() {
    macro_rules! run {
        ($class_name:ty) => {
            let fb = Arc::new(<$class_name>::new(5));
            let fb1 = fb.clone();
            let fb2 = fb.clone();
            let t1 = thread::spawn(move || { fb1.foo() });
            let t2 = thread::spawn(move || { fb2.bar() });
            t1.join().unwrap();
            t2.join().unwrap();
            println!();
        };
    }
    run!(FooBarAtomic);
    run!(FooBarMutex);
}

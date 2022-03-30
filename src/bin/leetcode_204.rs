//! 计数质数

pub fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    if n < 2 { return 0; }
    let mut h = vec![true; n];
    for i in 2..n {
        if h[i] {
            for j in (i * i..n).step_by(i) {
                h[j] = false;
            }
        }
    }
    (h.into_iter().filter(|x| *x).count() - 2) as i32
}


fn main() {
    assert_eq!(count_primes(10), 4);
    assert_eq!(count_primes(0), 0);
    assert_eq!(count_primes(1), 0);
}

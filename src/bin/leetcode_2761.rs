//! 和等于目标值的质数对

pub fn cal_prime(n: usize) -> Vec<bool> {
    let mut result = vec![];
    let mut is_prime = vec![true; n + 1];
    for i in 2..=n {
        if is_prime[i] { result.push(i); }
        for &p in &result {
            if i * p >= n { break; }
            is_prime[i * p] = false;
            if i % p == 0 { break; }
        }
    }
    is_prime
}

pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
    static mut PRIME: Vec<bool> = vec![];
    unsafe {
        if PRIME.is_empty() {
            PRIME = cal_prime(1e6 as usize);
        }
        let n = n as usize;
        // 如果是奇数，只可能是2和一个质数
        if n % 2 == 1 {
            return if n > 4 && PRIME[n - 2] { vec![vec![2, n as i32 - 2]] } else { vec![] };
        }
        let mut result = vec![];
        for i in 2..=n / 2 {
            if PRIME[i] && PRIME[n - i] {
                result.push(vec![i as i32, (n - i) as i32])
            }
        }
        result
    }
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(10), vec![vec![3, 7], vec![5, 5]]);
        assert_eq!(func(2), Vec::<Vec<i32>>::new());
    }
    test(find_prime_pairs);
}

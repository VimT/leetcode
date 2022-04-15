//! 质数排列

fn is_prime(num: i32) -> bool {
    for i in 2..=(num as f64).sqrt() as i32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn num_prime_arrangements(n: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let prime_cnt = (2..=n).filter(|x| is_prime(*x)).count() as i32;
    let not_cnt = n - prime_cnt;
    let mut result = 1;
    for i in 1..=prime_cnt {
        result = (result * i as i64) % MOD;
    }
    for i in 1..=not_cnt {
        result = (result * i as i64) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(1), 1);
        assert_eq!(func(5), 12);
        assert_eq!(func(100), 682289015);
    }
    test(num_prime_arrangements);
}

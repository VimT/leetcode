//! 将整数转换为两个无零整数的和

pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    'out: for i in 1..=n / 2 {
        let mut a = i;
        while a > 0 {
            if a % 10 == 0 { continue 'out; }
            a /= 10;
        }
        let mut b = n - i;
        while b > 0 {
            if b % 10 == 0 { continue 'out; }
            b /= 10;
        }
        return vec![i, n - i];
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<i32>) {
        assert_eq!(func(2), vec![1, 1]);
        assert_eq!(func(11), vec![2, 9]);
    }
    test(get_no_zero_integers);
}

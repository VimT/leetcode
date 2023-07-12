//! 拆分成最多数目的正偶数之和

pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
    if final_sum & 1 == 1 { return vec![]; }
    let mut result = vec![];
    let mut left = final_sum;

    let mut i = 2;
    while i < final_sum {
        left -= i;
        if left <= i {
            left += i;
            break;
        }
        result.push(i);
        i += 2;
    }
    result.push(left);
    result
}

fn main() {
    fn test(func: fn(final_sum: i64) -> Vec<i64>) {
        assert_eq!(func(12), vec![2, 4, 6]);
        assert_eq!(func(7), vec![]);
        assert_eq!(func(28), vec![2, 4, 6, 16]);
    }
    test(maximum_even_split);
}

//! 检查「好数组」

use leetcode::gcd::gcd3;

/// 裴蜀定理
/// ax+by=1 has solution x, y if gcd(a,b) = 1.
pub fn is_good_array(nums: Vec<i32>) -> bool {
    let mut g = 0;
    for num in nums {
        g = gcd3(num, g);
    }
    g == 1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![12, 5, 7, 23]), true);
        assert_eq!(func(vec![29, 6, 10]), true);
        assert_eq!(func(vec![3, 6]), false);
    }
    test(is_good_array);
}

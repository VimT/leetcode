//! 九坤-03. 数字默契考验

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { return b; }
    return gcd(b % a, a);
}

/// 最小公倍数
/// 另一种思路：如果两个数字 a 和 b “默契”，那么它们需要满足：从 a 和 b 中去掉所有的 2 和 3 的因子之后，它们会变成相同的数
pub fn min_operations(numbers: Vec<i32>) -> i32 {
    let mut a = numbers[0] as i64;
    for &number in &numbers[1..] {
        a = a / gcd(a as i64, number as i64) * number as i64;
    }

    let mut result = 0;
    for num in numbers {
        let mut gap = a / num as i64;
        while gap & 1 == 0 {
            gap >>= 1;
            result += 1;
        }
        while gap % 3 == 0 {
            gap /= 3;
            result += 1;
        }
        if gap != 1 {
            return -1;
        }
    }
    result
}

fn main() {
    assert_eq!(min_operations(vec![50, 75, 100]), 5);
    assert_eq!(min_operations(vec![10, 14]), -1);
}

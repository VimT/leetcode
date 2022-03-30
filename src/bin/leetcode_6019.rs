//! 替换数组中的非互质数

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b; }
    gcd(b % a, a)
}

pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for num in nums {
        result.push(num);
        while result.len() > 1 {
            let x = result[result.len() - 1];
            let y = result[result.len() - 2];
            let g = gcd(x, y);
            if g == 1 { break; }
            result.pop();
            *result.last_mut().unwrap() *= x / g;
        }
    }
    result
}

fn main() {
    assert_eq!(replace_non_coprimes(vec![287, 41, 49, 287, 899, 23, 23, 20677, 5, 825]), vec![2009, 20677, 825]);
    assert_eq!(replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]), vec![12, 7, 6]);
    assert_eq!(replace_non_coprimes(vec![2, 2, 1, 1, 3, 3, 3]), vec![2, 1, 1, 3]);
}

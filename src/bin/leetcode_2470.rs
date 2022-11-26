//! 最小公倍数为 K 的子数组数目

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        let mut lcm = 1;
        for j in i..len {
            lcm = lcm * nums[j] / gcd(lcm, nums[j]);
            if lcm == k {
                result += 1;
            } else if lcm > k {
                break;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![3, 6, 2, 7, 1], 6), 4);
        assert_eq!(func(vec![3], 2), 0);
    }
    test(subarray_lcm);
}

//! 连接二进制表示可形成的最大数值

pub fn max_good_number(nums: Vec<i32>) -> i32 {
    let bin_strs: Vec<String> = nums.iter().map(|&num| format!("{:b}", num)).collect();
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0]
    ];

    permutations.iter()
        .map(|perm| {
            let combined = format!("{}{}{}", bin_strs[perm[0]], bin_strs[perm[1]], bin_strs[perm[2]]);
            i32::from_str_radix(&combined, 2).unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

pub fn max_good_number2(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by(|a, b| {
        let len_a = 32 - a.leading_zeros();
        let len_b = 32 - b.leading_zeros();
        (b << len_a | a).cmp(&(a << len_b | b))
    });
    let mut result = 0;
    for x in nums {
        result = result << (32 - x.leading_zeros()) | x;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3]), 30);
        assert_eq!(func(vec![2, 8, 16]), 1296);
    }
    test(max_good_number);
    test(max_good_number2);
}

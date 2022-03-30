//! 数组中数字出现的次数
//! 数组中其他数字都出现两边，有两个数字出现一遍，找出这两个数字

/// 异或的结果中，如果某个二进制位=1，说明两个数的这个二进制位置 不同。所以根据这个二进制位，把nums分成=0，!=0的
/// 两组，再分别异或得到结果。
pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = 0;
    for &i in nums.iter() {
        ret ^= i;
    }
    let mut i = 1;
    while (i & ret) == 0 {
        i <<= 1;
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    for n in nums {
        if (i & n) == 1 {
            ans2 ^= n;
        } else {
            ans1 ^= n;
        }
    }

    vec![ans1, ans2]
}

fn main() {
    println!("{:?}", single_numbers(vec![1, 1, 2, 2, 3, 4]));
}

//! 按位与为零的三元组

pub fn count_triplets(nums: Vec<i32>) -> i32 {
    const MAX: i32 = 1 << 16;
    let mut cnt = vec![0; MAX as usize];
    let len = nums.len();
    for i in 0..len {
        for j in 0..len {
            cnt[(nums[i] & nums[j]) as usize] += 1;
        }
    }
    let mut result = 0;
    for i in 0..len {
        let full = MAX - 1 - nums[i];
        let mut j = full;
        while j > 0 {
            result += cnt[j as usize];
            // 遍历各种bit情况去计算: 移除最后一个数字
            // 这里得注意： 因为数字每次会改变，所以得基于原始值去与，否则会漏掉很多数字
            j = (j - 1) & full;
        }
        result += cnt[0];
    }
    result
}

fn main() {
    assert_eq!(count_triplets(vec![2, 1, 3]), 12);
    assert_eq!(count_triplets(vec![0, 0, 0]), 27);
}

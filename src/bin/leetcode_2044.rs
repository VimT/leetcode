//! 统计按位或能得到最大值的子集数目

pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for &i in &nums {
        sum |= i;
    }
    let mut result = 0;
    let mut v = Vec::with_capacity(1 << nums.len());
    v.push(0);
    for num in nums {
        let len = v.len();
        for j in 0..len {
            v.push(v[j] | num);
            if v[j] | num == sum {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(count_max_or_subsets(vec![3, 1]), 2);
    assert_eq!(count_max_or_subsets(vec![2, 2, 2]), 7);
    assert_eq!(count_max_or_subsets(vec![3, 2, 1, 5]), 6);
}
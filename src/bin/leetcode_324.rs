//! 摆动排序 II

use leetcode::algorithm;

pub fn wiggle_sort(nums: &mut Vec<i32>) {
    if nums.len() <= 1 {
        return;
    }
    nums.sort();
    let mid = (nums.len() - 1) / 2;
    let mut a = nums[0..=mid].to_vec();
    let mut b = nums[mid + 1..nums.len()].to_vec();
    if a[a.len() - 1] == b[0] {
        // 相同元素出现次数 >= (len(nums) + 1) / 2时，就需要反序
        if nums.iter().filter(|&&x| x == b[0]).count() >= (nums.len() + 1) / 2 {
            a.reverse();
            b.reverse();
        }
    }
    for i in 0..a.len() {
        nums[2 * i] = a[i];
        if i < b.len() {
            nums[2 * i + 1] = b[i];
        }
    }
}

pub fn wiggle_sort2(nums: &mut Vec<i32>) {
    if nums.len() <= 1 {
        return;
    }
    let mid = (nums.len() - 1) / 2;
    algorithm::nth_element(nums, mid);
    algorithm::three_way_partition(nums, nums[mid]);
    let a = nums[0..=mid].to_vec();
    let b = nums[mid + 1..nums.len()].to_vec();
    for i in 0..a.len() {
        nums[2 * i] = a[i];
        if i < b.len() {
            nums[2 * i + 1] = b[i];
        }
    }
}


fn main() {
    fn test(func: fn(nums: &mut Vec<i32>)) {
        let help = |mut nums: Vec<i32>| {
            func(&mut nums);
            nums
        };
        fn verify(nums: Vec<i32>) {
            let mut lt = true;
            for i in 1..nums.len() {
                if lt { if !(nums[i - 1] < nums[i]) { panic!(); } } else { if !(nums[i - 1] > nums[i]) { panic!(); } }
                lt = !lt;
            }
        }
        verify(help(vec![1, 5, 1, 1, 6, 4]));
        verify(help(vec![1, 3, 2, 2, 3, 1]));
    }
    test(wiggle_sort);
    test(wiggle_sort2);
}

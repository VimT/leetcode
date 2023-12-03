//! 交换得到字典序最小的数组

pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let mut ni: Vec<(i32, usize)> = nums.iter().copied().zip(0..).collect();
    ni.sort_unstable();
    let len = ni.len();
    let mut i = 0;
    while i < len {
        let mut j = i + 1;
        let mut pos_list = vec![ni[i].1];
        while j < len && ni[j].0 - ni[j - 1].0 <= limit {
            pos_list.push(ni[j].1);
            j += 1;
        }
        pos_list.sort_unstable();
        for (pos, k) in pos_list.into_iter().zip(i..) {
            nums[pos] = ni[k].0;
        }
        i = j;
    }
    nums
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, limit: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 5, 3, 9, 8], 2), vec![1, 3, 5, 8, 9]);
        assert_eq!(func(vec![1, 7, 6, 18, 2, 1], 3), vec![1, 6, 7, 18, 1, 2]);
        assert_eq!(func(vec![1, 7, 28, 19, 10], 3), vec![1, 7, 28, 19, 10]);
    }
    test(lexicographically_smallest_array);
}

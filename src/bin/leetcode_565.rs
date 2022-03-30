//! 数组嵌套

pub fn array_nesting(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut rev = vec![0; len];
    for (k, &v) in nums.iter().enumerate() {
        rev[v as usize] = k;
    }
    let mut vis = vec![false; len];
    let mut result = 0;
    for i in 0..len {
        if !vis[i] {
            let mut cur = 0;
            let mut start = i;
            while !vis[start] {
                vis[start] = true;
                start = nums[start] as usize;
                cur += 1;
            }
            start = rev[i];
            while !vis[start] {
                vis[start] = true;
                start = rev[start];
                cur += 1;
            }
            result = result.max(cur);
        }
    }
    result
}

fn main() {
    assert_eq!(array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    assert_eq!(array_nesting(vec![0, 1, 2]), 1);
}

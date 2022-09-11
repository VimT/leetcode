//! 裁剪数字后查询第 K 小的数字

use leetcode::svec;

pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    queries.iter().map(|query| {
        let k = query[0];
        let trim = query[1] as usize;
        let mut nums: Vec<(usize, Vec<u8>, usize)> = nums.iter().enumerate().map(|(idx, x)| {
            let x = x.as_bytes();
            let mut i = x.len() - trim;
            while i < x.len() && x[i] == b'0' { i += 1; }
            let arr = x[i..].to_vec();
            (arr.len(), arr, idx)
        }).collect();
        nums.sort_unstable();
        nums[k as usize - 1].2 as i32
    }).collect()
}

fn main() {
    assert_eq!(smallest_trimmed_numbers(svec!["87","70"], vec![vec![2,1],vec![1,2],vec![1,2],vec![1,1]]), vec![0, 1, 1, 1]);
    assert_eq!(smallest_trimmed_numbers(svec!["102","473","251","814"], vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]]), vec![2, 2, 1, 0]);
    assert_eq!(smallest_trimmed_numbers(svec!["24","37","96","04"], vec![vec![2, 1], vec![2, 2]]), vec![3, 0]);
}
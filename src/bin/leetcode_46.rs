//! 全排列

use leetcode::unorder;

pub fn permute_py(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut indices: Vec<usize> = (0..n).collect();
    let mut cycles: Vec<usize> = (1..=n).rev().collect();
    let mut ans = vec![nums.clone()];
    loop {
        let mut flag = true;
        for i in (0..n).rev() {
            cycles[i] -= 1;
            if cycles[i] == 0 {
                let tmp = indices[i];
                for i in i..indices.len() - 1 {
                    indices[i] = indices[i + 1];
                }
                indices[n - 1] = tmp;
                cycles[i] = n - i;
            } else {
                let j = cycles[i];
                indices.swap(i, n - j);
                let mut tmp = vec![];
                for &x in &indices {
                    tmp.push(nums[x]);
                }
                ans.push(tmp);
                flag = false;
                break;
            }
        }
        if flag {
            return ans;
        }
    }
}

pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &mut Vec<i32>, first: usize, ans: &mut Vec<Vec<i32>>, len: usize) {
        if first == len {
            ans.push(nums.to_vec());
        }
        for i in first..len {
            nums.swap(i, first);
            backtrack(nums, first + 1, ans, len);
            nums.swap(i, first);
        }
    }
    let len = nums.len();
    let mut ans = vec![];
    backtrack(&mut nums, 0, &mut ans, len);
    ans
}


pub fn permute_unique_best(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut ans = vec![nums];

    for position in 0..len - 1 {
        for i in 0..ans.len() {
            for j in position + 1..len {
                let mut tmp = ans[i].clone();
                tmp.swap(position, j);
                ans.push(tmp);
            }
        }
    }
    ans
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![1, 2, 3])), unorder(vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]));
        assert_eq!(unorder(func(vec![0, 1])), unorder(vec![vec![0, 1], vec![1, 0]]));
        assert_eq!(unorder(func(vec![1])), unorder(vec![vec![1]]));
    }
    test(permute);
    test(permute_py);
    test(permute_unique_best);
}

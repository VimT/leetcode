//! 矩阵中的和

pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
    for line in &mut nums {
        line.sort_unstable();
    }
    let m = nums.len();
    let n = nums[0].len();
    let mut result = 0;
    for i in 0..n {
        let mut max = 0;
        for j in 0..m {
            max = max.max(nums[j][i]);
        }
        result += max;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![7,2,1],vec![6,4,2],vec![6,5,3],vec![3,2,1]]), 15);
        assert_eq!(func(vec![vec![1]]), 1);
    }
    test(matrix_sum);
}

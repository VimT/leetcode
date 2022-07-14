//! 多个数组求交集

pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums.len();
    let mut cnt = vec![0; 1001];
    for num in nums {
        for n in num {
            cnt[n as usize] += 1;
        }
    }
    let mut result = vec![];
    for i in 0..1001 {
        if cnt[i] == len {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(intersection(vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]]), vec![3, 4]);
    assert_eq!(intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![]);
}

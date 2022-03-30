//! 查询后的偶数和

pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::with_capacity(queries.len());
    let mut cur_sum = 0;
    for &num in &nums {
        if num & 1 == 0 {
            cur_sum += num;
        }
    }
    for query in queries {
        let val = query[0];
        let idx = query[1] as usize;
        let after = nums[idx] + val;
        if nums[idx] & 1 == 0 {
            if after & 1 == 0 {
                cur_sum = cur_sum - nums[idx] + after;
            } else {
                cur_sum -= nums[idx];
            }
        } else {
            if after & 1 == 0 {
                cur_sum += after;
            }
        }
        nums[idx] = after;
        result.push(cur_sum);
    }
    result
}

fn main() {
    assert_eq!(sum_even_after_queries(vec![1, 2, 3, 4], vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]), vec![8, 6, 2, 4]);
    assert_eq!(sum_even_after_queries(vec![1], vec![vec![4, 0]]), vec![0]);
}

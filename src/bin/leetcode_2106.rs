//! 摘水果

pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
    let cur_idx = fruits.binary_search_by_key(&(start_pos), |x| x[0]);
    let left_idx = fruits.binary_search_by_key(&(start_pos - k), |x| x[0]).unwrap_or_else(|x| x);
    let has_left = left_idx < fruits.len() && fruits[left_idx][0] < start_pos && start_pos - fruits[left_idx][0] <= k;
    let right_idx = fruits.binary_search_by_key(&(start_pos + k), |x| x[0]).unwrap_or_else(|x| if x > 0 { x - 1 } else { x });
    let has_right = fruits[right_idx][0] > start_pos && fruits[right_idx][0] - start_pos <= k;
    if !has_left && !has_right {
        return if cur_idx.is_ok() { fruits[cur_idx.unwrap()][1] } else { 0 };
    }
    let mut result = 0;
    if has_left {
        let mut cur = 0;
        for i in left_idx..=cur_idx.unwrap_or_else(|x| x - 1) {
            cur += fruits[i][1];
        }
        result = result.max(cur);
    }
    if has_right {
        let mut cur = 0;
        for i in cur_idx.unwrap_or_else(|x| x)..=right_idx {
            cur += fruits[i][1];
        }
        result = result.max(cur);
    }

    if has_left && has_right {
        let mut cur_sum = 0;
        let cur_idx_right = cur_idx.unwrap_or_else(|x| x);
        let mut right = left_idx;
        for _ in left_idx..cur_idx_right {
            cur_sum += fruits[right][1];
            right += 1;
        }
        for left in left_idx..=cur_idx_right {
            while right < left {
                cur_sum += fruits[right][1];
                right += 1;
            }
            while right <= right_idx && (((start_pos - fruits[left][0]).abs() * 2 + (fruits[right][0] - start_pos).abs() <= k) || ((fruits[right][0] - start_pos).abs() * 2 + (start_pos - fruits[left][0]).abs()) <= k) {
                cur_sum += fruits[right][1];
                right += 1;
                result = result.max(cur_sum);
            }
            cur_sum -= fruits[left][1];
        }
    }
    result
}

fn main() {
    assert_eq!(max_total_fruits(vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4), 9);
    assert_eq!(max_total_fruits(vec![vec![1, 9], vec![2, 10], vec![3, 1], vec![5, 6], vec![6, 3], vec![8, 2], vec![9, 2], vec![11, 4], vec![18, 10], vec![22, 8], vec![25, 2], vec![26, 2], vec![30, 4], vec![31, 5], vec![33, 9], vec![34, 1], vec![39, 10]], 19, 9), 22);
    assert_eq!(max_total_fruits(vec![vec![200000, 10000]], 0, 100000), 0);
    assert_eq!(max_total_fruits(vec![vec![200000, 10000]], 0, 200000), 10000);
    assert_eq!(max_total_fruits(vec![vec![200000, 10000]], 400000, 100000), 0);
    assert_eq!(max_total_fruits(vec![vec![0, 10000]], 200000, 200000), 10000);
    assert_eq!(max_total_fruits(vec![vec![0, 7], vec![7, 4], vec![9, 10], vec![12, 6], vec![14, 8], vec![16, 5], vec![17, 8], vec![19, 4], vec![20, 1], vec![21, 3], vec![24, 3], vec![25, 3], vec![26, 1], vec![28, 10], vec![30, 9], vec![31, 6], vec![32, 1], vec![37, 5], vec![40, 9]], 21, 30), 71);
    assert_eq!(max_total_fruits(vec![vec![200000, 10000]], 200000, 0), 10000);
    assert_eq!(max_total_fruits(vec![vec![0, 9], vec![4, 1], vec![5, 7], vec![6, 2], vec![7, 4], vec![10, 9]], 5, 4), 14);
    assert_eq!(max_total_fruits(vec![vec![0, 3], vec![6, 4], vec![8, 5]], 3, 2), 0);
}

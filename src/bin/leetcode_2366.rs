//! 将数组排序的最少替换次数

/// 贪心，从后往前看最小值，如果当前数>最小值，就需要切割
pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
    let mut cur_min = *nums.last().unwrap();
    let mut result = 0;
    let len = nums.len();
    for i in (0..len).rev() {
        let num = nums[i];
        if num <= cur_min {
            cur_min = num;
        } else {
            // 均匀切割可以使 切割后最小数最大，切割成 cur_num份 可以让最小数 <= cur_min。
            let mut cut_num = num / cur_min;
            if num % cur_min != 0 {
                cut_num += 1;
            }
            cur_min = num / cut_num;
            result += (cut_num - 1) as i64;
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_replacement(vec![3, 9, 3]), 2);
    assert_eq!(minimum_replacement(vec![1, 2, 3, 4, 5]), 0);
}

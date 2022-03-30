//! K 个不同整数的子数组

/// to another question. subarray with max k distinct. then answer = func(k) - func(k-1)
pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
    fn subarrays_with_max_k_distinct(a: &Vec<i32>, k: i32) -> i32 {
        let len = a.len();
        let mut ans = 0;
        let k = k as usize;
        let mut freq = vec![0; len + 1];
        let mut left = 0;
        let mut right = 0;
        let mut count = 0;
        while right < len {
            if freq[a[right] as usize] == 0 {
                count += 1;
            }
            freq[a[right] as usize] += 1;
            right += 1;
            while count > k {
                freq[a[left] as usize] -= 1;
                if freq[a[left] as usize] == 0 {
                    count -= 1;
                }
                left += 1;
            }
            ans += right - left;
        }
        ans as i32
    }
    return subarrays_with_max_k_distinct(&a, k) - subarrays_with_max_k_distinct(&a, k - 1);
}

fn main() {
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
}

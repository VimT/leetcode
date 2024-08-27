//! 统计满足 K 约束的子字符串数量 II

pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = s.len();
    let s = s.as_bytes();
    let mut left = vec![0; n];  // left[i]=j，表示 s[j..i] 满足条件
    let mut pre = vec![0; n + 1]; // pre[i] 表示 i+1 - left[i] 的前缀和
    let mut l = 0;
    let mut cnt = [0; 2];
    for (i, &c) in s.iter().enumerate() {
        cnt[(c - b'0') as usize] += 1;
        while cnt[0] > k && cnt[1] > k {
            cnt[(s[l] - b'0') as usize] -= 1;
            l += 1;
        }
        left[i] = l;
        pre[i + 1] = pre[i] + i + 1 - l;
    }
    queries.into_iter().map(|query| {
        let (l, r) = (query[0] as usize, query[1] as usize);
        // return (l..=r).map(|i| i - left[i].max(l) + 1).sum::<usize>() as i64;  // 这样写会超时
        // left[i] 具有单调性，使用二分查找优化
        // i 在 [j, r] 时，都是 i - left[i] + 1 ，使用前缀和
        // i 在 [l, j) 时，都是 i - l + 1 ，相当于 [1, 2, 3, ..., j-l] 的和
        let j = left[l..=r].binary_search_by(|x| x.cmp(&l).then(std::cmp::Ordering::Greater)).unwrap_err() + l;
        (pre[r + 1] - pre[j] + (j - l + 1) * (j - l) / 2) as i64
    }).collect()
}

fn main() {
    fn test(func: fn(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64>) {
        assert_eq!(func(String::from("0001111"), 2, vec![vec![0, 6]]), vec![26]);
        assert_eq!(func(String::from("010101"), 1, vec![vec![0, 5], vec![1, 4], vec![2, 3]]), vec![15, 9, 3]);
    }
    test(count_k_constraint_substrings);
}

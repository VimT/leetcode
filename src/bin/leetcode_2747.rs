//! 统计没有收到请求的服务器数目

pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
    logs.sort_unstable_by_key(|x| x[1]);
    let qlen = queries.len();
    let mut result = vec![0; qlen];
    let mut qi: Vec<(i32, usize)> = queries.into_iter().zip(0..).collect();
    qi.sort_unstable();
    let len = logs.len();
    let mut server = vec![0; n as usize + 1];
    let mut cnt = 0;
    let mut i = 0;
    let mut j = 0;
    for (q, idx) in qi {
        while j < len && logs[j][1] <= q {
            if server[logs[j][0] as usize] == 0 { cnt += 1; }
            server[logs[j][0] as usize] += 1;
            j += 1;
        }
        while i < len && logs[i][1] < q - x {
            server[logs[i][0] as usize] -= 1;
            if server[logs[i][0] as usize] == 0 { cnt -= 1; }
            i += 1;
        }
        result[idx] = n - cnt;
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(6, vec![vec![1, 21]], 10, vec![24, 35, 28, 26, 20, 25, 16, 31, 12]), vec![5, 6, 5, 5, 6, 5, 6, 5, 6]);
        assert_eq!(func(3, vec![vec![1, 3], vec![2, 6], vec![1, 5]], 5, vec![10, 11]), vec![1, 2]);
        assert_eq!(func(3, vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1]], 2, vec![3, 4]), vec![0, 1]);
    }
    test(count_servers);
}

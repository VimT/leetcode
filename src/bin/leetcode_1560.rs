//! 圆形赛道上经过次数最多的扇区

pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    let mut cnt = vec![0; n as usize];
    let mut cur = rounds[0] - 1;
    for &i in &rounds[1..] {
        let i = i - 1;
        while (cur + n - 1) % n != i {
            cnt[cur as usize] += 1;
            cur += 1;
            cur %= n;
        }
    }
    let mx = *cnt.iter().max().unwrap();
    (1..=n).filter(|&x| cnt[x as usize - 1] == mx).collect()
}

/// 结果只会和 起点、终点有关。从起点沿着逆时针方向走到终点的这部分扇区，就是经过次数最多的扇区。
pub fn most_visited2(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    let start = rounds[0];
    let end = rounds.last().copied().unwrap();
    let mut result = vec![];
    if end < start {
        for i in 1..=end { result.push(i); }
        for i in start..=n { result.push(i); }
    } else {
        for i in start..=end { result.push(i); }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, rounds: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(4, vec![1, 3, 1, 2]), vec![1, 2]);
        assert_eq!(func(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]), vec![2]);
        assert_eq!(func(7, vec![1, 3, 5, 7]), vec![1, 2, 3, 4, 5, 6, 7]);
    }
    test(most_visited);
    test(most_visited2);
}

//! 有相同颜色的相邻元素数目

pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = n as usize;
    let mut a = vec![0; len];
    let mut cnt = 0;
    queries.into_iter().map(|q| {
        let i = q[0] as usize;
        let c = q[1];
        if a[i] > 0 {
            if i > 0 && a[i] == a[i - 1] { cnt -= 1; }
            if i + 1 < len && a[i] == a[i + 1] { cnt -= 1; }
        }
        a[i] = c;
        if i > 0 && a[i] == a[i - 1] { cnt += 1; }
        if i + 1 < len && a[i] == a[i + 1] { cnt += 1; }
        cnt
    }).collect()
}

fn main() {
    fn test(func: fn(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(4, vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]]), vec![0, 1, 1, 0, 2]);
        assert_eq!(func(1, vec![vec![0, 100000]]), vec![0]);
    }
    test(color_the_array);
}

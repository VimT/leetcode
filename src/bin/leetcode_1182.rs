//! 与目标颜色间的最短距离

pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = colors.len();
    let mut left = vec![[0; 3]; len];
    let mut right = vec![[0; 3]; len];
    let mut last = [i32::MIN / 2; 3];
    for i in 0..len {
        let color = colors[i] as usize - 1;
        last[color] = i as i32;
        left[i] = last.clone();
    }
    last = [i32::MAX / 2; 3];
    for i in (0..len).rev() {
        let color = colors[i] as usize - 1;
        last[color] = i as i32;
        right[i] = last.clone();
    }
    queries.into_iter().map(|query| {
        let i = query[0];
        let color = query[1] as usize - 1;
        let result = (i - left[i as usize][color]).min(right[i as usize][color] - i);
        if result >= i32::MAX / 3 { -1 } else { result }
    }).collect()
}

fn main() {
    fn test(func: fn(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 1, 2, 1, 3, 2, 2, 3, 3], vec![vec![1, 3], vec![2, 2], vec![6, 1]]), vec![3, 0, 3]);
        assert_eq!(func(vec![1, 2], vec![vec![0, 3]]), vec![-1]);
    }
    test(shortest_distance_color);
}

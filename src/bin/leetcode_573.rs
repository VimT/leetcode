//! 松鼠模拟

pub fn min_distance(_: i32, _: i32, tree: Vec<i32>, squirrel: Vec<i32>, nuts: Vec<Vec<i32>>) -> i32 {
    fn dis(a: (i32, i32), b: (i32, i32)) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }
    let tree = (tree[0], tree[1]);
    let sq = (squirrel[0], squirrel[1]);
    let mut sum = 0;
    for nut in &nuts {
        sum += 2 * dis((nut[0], nut[1]), tree);
    }
    let mut result = i32::MAX;
    for nut in nuts {
        let nut = (nut[0], nut[1]);
        result = result.min(sum - dis(nut, tree) + dis(sq, nut));
    }
    result
}

fn main() {
    fn test(func: fn(height: i32, width: i32, tree: Vec<i32>, squirrel: Vec<i32>, nuts: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(5, 7, vec![2, 2], vec![4, 4], vec![vec![3, 0], vec![2, 5]]), 12);
        assert_eq!(func(1, 3, vec![0, 1], vec![0, 0], vec![vec![0, 2]]), 3);
    }
    test(min_distance);
}

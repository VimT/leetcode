//! 最大化网格图中正方形空洞的面积

pub fn maximize_square_hole_area(_: i32, _: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
    fn max_side(mut v: Vec<i32>) -> i32 {
        v.sort_unstable();
        let len = v.len();
        let mut result = 2;
        let mut cur = 2;
        for i in 1..len {
            if v[i] == v[i - 1] + 1 {
                cur += 1;
                result = result.max(cur);
            } else {
                cur = 2;
            }
        }
        result
    }
    let side = max_side(h_bars).min(max_side(v_bars));
    side * side
}

fn main() {
    fn test(func: fn(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32) {
        assert_eq!(func(3, 2, vec![3,2,4], vec![3,2]), 9);
        assert_eq!(func(2, 1, vec![2, 3], vec![2]), 4);
        assert_eq!(func(1, 1, vec![2], vec![2]), 4);
        assert_eq!(func(2, 3, vec![2, 3], vec![2, 3, 4]), 9);
    }
    test(maximize_square_hole_area);
}

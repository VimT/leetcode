//! 切割后面积最大的蛋糕

pub fn max_area(h: i32, w: i32, mut hh: Vec<i32>, mut vv: Vec<i32>) -> i32 {
    hh.sort_unstable();
    vv.sort_unstable();
    hh.push(h);
    vv.push(w);
    let mut maxh = hh[0];
    let mut maxv = vv[0];
    for i in 0..hh.len() - 1 {
        maxh = maxh.max(hh[i + 1] - hh[i]);
    }
    for i in 0..vv.len() - 1 {
        maxv = maxv.max(vv[i + 1] - vv[i]);
    }
    (maxh as u64 * maxv as u64 % (1e9 as u64 + 7)) as i32
}

fn main() {
    fn test(func: fn(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32) {
        assert_eq!(func(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
        assert_eq!(func(5, 4, vec![3, 1], vec![1]), 6);
        assert_eq!(func(5, 4, vec![3], vec![3]), 9);
    }
    test(max_area);
}

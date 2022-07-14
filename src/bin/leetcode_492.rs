//! 构造矩形

pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let mut w = (area as f64).sqrt() as i32;
    while area % w != 0 {
        w -= 1;
    }
    return vec![area / w, w];
}


fn main() {
    fn test(func: fn(area: i32) -> Vec<i32>) {
        assert_eq!(func(4), vec![2, 2]);
        assert_eq!(func(37), vec![37, 1]);
        assert_eq!(func(122122), vec![427, 286]);
    }
    test(construct_rectangle);
}

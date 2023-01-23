//! 不浪费原料的汉堡制作方案

pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    let x2 = tomato_slices - cheese_slices * 2;
    let y = (cheese_slices * 2 - x2) / 2;
    if x2 >= 0 && y >= 0 && x2 & 1 == 0 {
        return vec![x2 / 2, y];
    }
    vec![]
}

fn main() {
    fn test(func: fn(tomato_slices: i32, cheese_slices: i32) -> Vec<i32>) {
        assert_eq!(func(16, 7), vec![1, 6]);
        assert_eq!(func(17, 4), vec![]);
        assert_eq!(func(4, 17), vec![]);
        assert_eq!(func(0, 0), vec![0, 0]);
    }
    test(num_of_burgers);
}

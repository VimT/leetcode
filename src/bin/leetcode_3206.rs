//! 交替组 I

pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let len = colors.len();
    let mut result = 0;
    for i in 0..len {
        if colors[(i + len - 1) % len] != colors[i] && colors[i] != colors[(i + 1) % len] {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(colors: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 1]), 0);
        assert_eq!(func(vec![0, 1, 0, 0, 1]), 3);
    }
    test(number_of_alternating_groups);
}

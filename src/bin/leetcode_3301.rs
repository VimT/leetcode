//! 高度互不相同的最大塔高和

pub fn maximum_total_sum(mut maximum_height: Vec<i32>) -> i64 {
    maximum_height.sort_unstable();
    let len = maximum_height.len();
    let mut current = maximum_height[len - 1];
    let mut result = 0;
    for num in maximum_height.into_iter().rev() {
        if current == 0 { return -1; }
        if num < current {
            current = num;
        }
        result += current as i64;
        current -= 1;
    }
    result
}

fn main() {
    fn test(func: fn(maximum_height: Vec<i32>) -> i64) {
        assert_eq!(func(vec![2, 3, 4, 3]), 10);
        assert_eq!(func(vec![15, 10]), 25);
        assert_eq!(func(vec![2, 2, 1]), -1);
    }
    test(maximum_total_sum);
}

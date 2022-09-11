//! 最多可以买到的苹果数量

pub fn max_number_of_apples(mut weight: Vec<i32>) -> i32 {
    weight.sort_unstable();
    let mut sum = 0;
    for i in 0..weight.len() {
        sum += weight[i];
        if sum > 5000 {
            return i as i32;
        }
    }
    weight.len() as i32
}

fn main() {
    fn test(func: fn(weight: Vec<i32>) -> i32) {
        assert_eq!(func(vec![100, 200, 150, 1000]), 4);
        assert_eq!(func(vec![900, 950, 800, 1000, 700, 800]), 5);
    }
    test(max_number_of_apples);
}

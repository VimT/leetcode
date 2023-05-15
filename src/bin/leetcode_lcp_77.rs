//! 符文储备

pub fn rune_reserve(mut runes: Vec<i32>) -> i32 {
    runes.sort_unstable();
    let mut result = 1;
    let len = runes.len();
    let mut j = 0;
    for i in 1..len {
        if runes[i] - runes[i-1] > 1 {
            j = i;
        } else {
            result = result.max(i - j + 1);
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(runes: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 5, 4, 1, 7]), 3);
        assert_eq!(func(vec![1, 1, 3, 3, 2, 4]), 6);
    }
    test(rune_reserve);
}

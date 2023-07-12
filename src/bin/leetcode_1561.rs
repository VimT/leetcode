//! 你可以获得的最大硬币数目

pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable();
    let mut i = piles.len() as i32 - 2;
    let mut result = 0;
    for _ in 0..piles.len() / 3 {
        result += piles[i as usize];
        i -= 2;
    }
    result
}

fn main() {
    fn test(func: fn(piles: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(func(vec![2, 4, 5]), 4);
        assert_eq!(func(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
    test(max_coins);
}

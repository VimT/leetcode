//! 种花问题

pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    let len = flowerbed.len();
    for i in 0..len {
        if flowerbed[i] == 0 {
            let left_empty = if i > 0 { flowerbed[i - 1] == 0 } else { true };
            let right_empty = if i + 1 < len { flowerbed[i + 1] == 0 } else { true };
            if left_empty && right_empty {
                flowerbed[i] = 1;
                n -= 1;
            }
        }
    }
    n <= 0
}

fn main() {
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
}

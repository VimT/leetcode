//! 必须拿起的最小连续卡牌数

use std::collections::HashMap;

pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut last = HashMap::new();
    let mut result = i32::MAX;
    for i in 0..cards.len() {
        if let Some(last_idx) = last.get(&cards[i]) {
            result = result.min((i - last_idx + 1) as i32)
        }
        last.insert(cards[i], i);
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    assert_eq!(minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
    assert_eq!(minimum_card_pickup(vec![1, 0, 5, 3]), -1);
}

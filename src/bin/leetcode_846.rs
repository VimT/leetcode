//! 一手顺子

use std::collections::BTreeMap;

pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    let len = hand.len();
    let group_size = group_size as usize;
    if len % group_size != 0 { return false; }
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    for pai in hand {
        *map.entry(pai).or_default() += 1;
    }
    for _ in 0..len / group_size {
        let start = *map.range(..).next().unwrap().0;
        for cur in start..start + group_size as i32 {
            if map.contains_key(&cur) {
                *map.get_mut(&cur).unwrap() -= 1;
                if map[&cur] == 0 {
                    map.remove(&cur);
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn main() {
    assert_eq!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
    assert_eq!(is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
}

//! 祖玛游戏

use std::collections::HashMap;

pub fn find_min_step(board: String, hand: String) -> i32 {
    fn dfs(a: Vec<u8>, b: &Vec<u8>, cur: i32, cache: &mut HashMap<Vec<u8>, i32>) -> i32 {
        if a.is_empty() { return 0; }
        if let Some(&v) = cache.get(&a) {
            return v;
        }
        let mut result = 1000;
        let len = a.len();
        for i in 0..b.len() {
            if cur >> i & 1 == 1 { continue; }
            let nxt = (1 << i) | cur;
            for j in 0..=len {
                let mut ok = false;
                if j > 0 && j < len && a[j] == a[j - 1] && a[j - 1] != b[i] {
                    ok = true;
                }
                if j < len && a[j] == b[i] {
                    ok = true;
                }
                if !ok { continue; }
                let mut na = Vec::with_capacity(len + 1);
                na.extend_from_slice(&a[..j]);
                na.push(b[i]);
                na.extend_from_slice(&a[j..]);
                let mut k = j;
                while k < na.len() {
                    let c = na[k];
                    let mut l = k as i32;
                    let mut r = k as i32;
                    while l >= 0 && na[l as usize] == c { l -= 1; }
                    while r < na.len() as i32 && na[r as usize] == c { r += 1; }
                    if r - l - 1 >= 3 {
                        na.drain((l + 1) as usize..r as usize);
                        k = if l >= 0 { l as usize } else { r as usize };
                    } else {
                        break;
                    }
                }
                result = result.min(dfs(na, b, nxt, cache) + 1);
            }
        }
        cache.insert(a.clone(), result);
        result
    }
    let board = board.as_bytes();
    let hand = hand.as_bytes();
    let result = dfs(board.to_vec(), &hand.to_vec(), 1 << hand.len(), &mut HashMap::new());
    if result >= 100 { -1 } else { result }
}

fn main() {
    // RRWWRRBBRWR
    assert_eq!(find_min_step("RRWWRRBBRR".to_string(), "WB".to_string()), 2);
    assert_eq!(find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()), 2);
    assert_eq!(find_min_step("WRRBBW".to_string(), "RB".to_string()), -1);
    assert_eq!(find_min_step("G".to_string(), "GGGGG".to_string()), 2);
    assert_eq!(find_min_step("RBYYBBRRB".to_string(), "YRBGB".to_string()), 3);
}

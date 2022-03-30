//! 秋叶收藏集

pub fn minimum_operations(leaves: String) -> i32 {
    let leaves = leaves.as_bytes();
    let len = leaves.len();
    let mut g = if leaves[0] == b'y' { 1 } else { -1 };
    let mut gmin = g;
    let mut ans = i32::MAX;
    for i in 1..len {
        let is_yellow = leaves[i] == b'y';
        g += 2 * is_yellow as i32 - 1;
        if i != len - 1 {
            ans = ans.min(gmin - g);
        }
        gmin = gmin.min(g);
    }
    ans + (g + len as i32) / 2
}


fn main() {
    assert_eq!(minimum_operations("rrryyyrryyyrr".to_string()), 2);
    assert_eq!(minimum_operations("ryr".to_string()), 0);
}

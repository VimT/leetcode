//! Z 字形变换

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s; }
    let mut ans = vec![vec![]; (num_rows as usize).min(s.len())];
    let mut row = 0;
    let mut downing = false;
    for &c in s.as_bytes() {
        ans[row].push(c);
        if row == (num_rows - 1) as usize || row == 0 { downing = !downing; }
        if downing { row += 1; } else { row -= 1; }
    }

    ans.into_iter().map(|x| unsafe { String::from_utf8_unchecked(x) }).collect()
}


fn main() {
    assert_eq!(convert(String::from("PAYPALISHIRING"), 3), "PAHNAPLSIIGYIR");
    assert_eq!(convert(String::from("PAYPALISHIRING"), 4), "PINALSIGYAHRPI");
    assert_eq!(convert(String::from("A"), 1), "A");
}

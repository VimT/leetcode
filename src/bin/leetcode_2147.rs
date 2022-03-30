//! 分隔长廊的方案数

pub fn number_of_ways(corridor: String) -> i32 {
    const MOD: usize = 1e9 as usize + 7;
    let s = corridor.as_bytes();
    let mut seat_num = 0;
    for &ch in s {
        if ch == b'S' { seat_num += 1; }
    }
    if seat_num & 1 == 1 || seat_num == 0 { return 0; }
    let mut result = 1;
    let mut i = 0;
    let len = s.len();
    while i < len {
        let mut seat_cnt = 0;
        while i < len && seat_cnt < 2 {
            if s[i] == b'S' {
                seat_cnt += 1;
            }
            i += 1;
        }
        let second_seat = i - 1;
        let mut next_seat = i;
        while next_seat < len && s[next_seat] != b'S' {
            next_seat += 1;
        }
        if next_seat < len {
            result = (result * (next_seat - second_seat)) % MOD;
        }
        i = next_seat;
    }
    result as i32
}

fn main() {
    assert_eq!(number_of_ways(String::from("SPPSSSSPPS")), 1);
    assert_eq!(number_of_ways(String::from("PPSPSP")), 1);
    assert_eq!(number_of_ways(String::from("SSPPSPSPPPP")), 3);
    assert_eq!(number_of_ways(String::from("S")), 0);
    assert_eq!(number_of_ways(String::from("P")), 0);
}

//! N 天后的牢房


pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    let mut repeat = vec![];
    let mut set = [false; 256];
    #[inline]
    fn next(cur: u8) -> u8 {
        (!((cur >> 1) ^ (cur << 1))) & (0b01111110)
    }
    let mut cur = 0;
    for i in 0..8 {
        if cells[i] == 1 {
            cur |= 1 << i;
        }
    }
    let mut tmp = next(cur);
    for _ in 0..n {
        if set[tmp as usize] {
            break;
        }
        set[tmp as usize] = true;
        repeat.push(tmp);
        tmp = next(tmp);
    }
    let period = repeat.len();
    let day = (n as usize - 1) % period;
    let rnum = repeat[day];
    let mut result = vec![0; 8];
    for i in 0..8 {
        result[i] = (rnum >> i & 1) as i32;
    }
    result
}

fn main() {
    assert_eq!(prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000), vec![0, 0, 1, 1, 1, 1, 1, 0]);
    assert_eq!(prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7), vec![0, 0, 1, 1, 0, 0, 0, 0]);
}

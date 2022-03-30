//! 丑数 II

static mut UGLY: [i32; 1690] = [0; 1690];

pub fn nth_ugly_number(n: i32) -> i32 {
    unsafe {
        if UGLY[0] == 0 {
            UGLY[0] = 1;
            let mut num2 = 0;
            let mut num3 = 0;
            let mut num5 = 0;
            for i in 1..1690 {
                let next = (2 * UGLY[num2]).min(3 * UGLY[num3]).min(5 * UGLY[num5]);
                UGLY[i] = next;
                if next == 2 * UGLY[num2] {
                    num2 += 1;
                }
                if next == 3 * UGLY[num3] {
                    num3 += 1;
                }
                if next == 5 * UGLY[num5] {
                    num5 += 1;
                }
            }
        }
        UGLY[n as usize - 1]
    }
}


fn main() {
    assert_eq!(nth_ugly_number(10), 12);
    assert_eq!(nth_ugly_number(1), 1);
}

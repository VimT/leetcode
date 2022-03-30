//! 一个区间内所有数乘积的缩写

pub fn abbreviate_product(left: i32, right: i32) -> String {
    let mut pre = 1;
    let mut suf = 1;
    let mut c = 0;
    let mut val = 1;
    const MOD: u64 = 1e10 as u64;
    const MAXVAL: u64 = 1e12 as u64;
    const MAXPRE: u64 = 1e13 as u64;
    for i in left as u64..=right as u64 {
        pre *= i;
        suf *= i;
        while pre > MAXPRE { pre /= 10; }
        while suf % 10 == 0 {
            suf /= 10;
            c += 1;
        }
        suf %= MOD;
        if val <= MAXVAL {
            val *= i;
            while val % 10 == 0 {
                val /= 10;
            }
        }
    }
    let val = val.to_string();
    if val.len() <= 10 { return format!("{}e{}", val, c); }
    let suf = suf.to_string();
    format!("{}...{:05}e{}", &pre.to_string()[..5], &suf[5.max(suf.len()) - 5..], c)
}

fn main() {
    assert_eq!(abbreviate_product(1, 4), String::from("24e0"));
    assert_eq!(abbreviate_product(2, 11), String::from("399168e2"));
    assert_eq!(abbreviate_product(999998, 1000000), String::from("99999...00002e6"));
    assert_eq!(abbreviate_product(256, 65535), String::from("23510...78528e16317"));
}

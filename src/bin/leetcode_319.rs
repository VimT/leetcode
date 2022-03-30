//! 灯泡开关

/// x = n^2/2
pub fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}


fn main() {
    assert_eq!(bulb_switch(3), 1);
    assert_eq!(bulb_switch(0), 0);
    assert_eq!(bulb_switch(1), 1);
}

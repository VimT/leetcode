//! 重新排序得到 2 的幂

static mut TABLE: Option<Vec<Vec<u8>>> = None;

pub fn reordered_power_of2(n: i32) -> bool {
    unsafe {
        if TABLE == None {
            let mut result = vec![];
            for i in 0..32 {
                let mut s = (1 << i).to_string().as_bytes().to_vec();
                s.sort_unstable();
                result.push(s);
            }
            result.sort_unstable();
            TABLE = Some(result);
        }
        let mut s = n.to_string().as_bytes().to_vec();
        s.sort_unstable();
        TABLE.as_ref().unwrap().binary_search(&s).is_ok()
    }
}

fn main() {
    assert!(reordered_power_of2(1));
    assert!(!reordered_power_of2(10));
    assert!(!reordered_power_of2(24));
    assert!(reordered_power_of2(46));
    assert!(reordered_power_of2(16));
}


//! IP 到 CIDR

pub fn ip_to_cidr(ip: String, n: i32) -> Vec<String> {
    let ip_chunk: Vec<u32> = ip.split(".").map(|x| x.parse::<u32>().unwrap()).collect();
    let start = ip_chunk[0] << 24 | ip_chunk[1] << 16 | ip_chunk[2] << 8 | ip_chunk[3];
    let end = start + n as u32;
    let mut cur = start;
    const MASK: u32 = 255;
    fn num2cidr(num: u32, prefix: u32) -> String {
        format!("{}.{}.{}.{}/{}", num >> 24 & MASK, num >> 16 & MASK, num >> 8 & MASK, num & MASK, prefix)
    }
    let mut result = vec![];
    while cur < end {
        let tail_zero = cur.trailing_zeros().min(31);
        for i in (0..=tail_zero).rev() {
            if cur | ((1 << i) - 1) < end {
                result.push(num2cidr(cur, 32 - i));
                cur = cur | ((1 << i) - 1);
                break;
            }
        }
        cur += 1;
    }
    result
}

fn main() {
    fn test(func: fn(ip: String, n: i32) -> Vec<String>) {
        assert_eq!(func(String::from("0.0.0.0"), 1), vec!["0.0.0.0/32"]);
        assert_eq!(func(String::from("117.145.102.62"), 8), vec!["117.145.102.62/31", "117.145.102.64/30", "117.145.102.68/31"]);
        assert_eq!(func(String::from("255.0.0.7"), 10), vec!["255.0.0.7/32", "255.0.0.8/29", "255.0.0.16/32"]);
    }
    test(ip_to_cidr);
}

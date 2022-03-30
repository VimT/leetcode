//! 比较版本号

pub fn compare_version(version1: String, version2: String) -> i32 {
    let v1 = version1.split(".").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let v2 = version2.split(".").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for i in 0..v1.len().max(v2.len()) {
        let n1 = v1.get(i).unwrap_or(&0);
        let n2 = v2.get(i).unwrap_or(&0);
        if n1 < n2 {
            return -1;
        } else if n1 > n2 {
            return 1;
        }
    }
    0
}


fn main() {
    assert_eq!(compare_version(String::from("1.01"), String::from("1.001")), 0);
    assert_eq!(compare_version(String::from("1.0"), String::from("1.0.0")), 0);
    assert_eq!(compare_version(String::from("0.1"), String::from("1.1")), -1);
}

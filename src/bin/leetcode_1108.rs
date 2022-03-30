//! IP 地址无效化

pub fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

fn main() {
    fn test(func: fn(address: String) -> String) {
        assert_eq!(func(String::from("1.1.1.1")), String::from("1[.]1[.]1[.]1"));
        assert_eq!(func(String::from("255.100.50.0")), String::from("255[.]100[.]50[.]0"));
    }
    test(defang_i_paddr);
}

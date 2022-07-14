//! 字符串中最多数目的子字符串

pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let s = text.as_bytes();
    let p = pattern.as_bytes();
    if p[0] == p[1] {
        let cnt = s.iter().filter(|x| **x == p[0]).count();
        return ((cnt + 1) * cnt / 2) as i64;
    }
    let mut result = 0;
    let mut p1cnt = 0;
    let mut p2cnt = 0;
    for &ch in s {
        if ch == p[0] {
            p1cnt += 1;
        } else if ch == p[1] {
            p2cnt += 1;
            result += p1cnt;
        }
    }
    result + p1cnt.max(p2cnt)
}

fn main() {
    assert_eq!(maximum_subsequence_count(String::from("vnedkpkkyxelxqptfwuzcjhqmwagvrglkeivowvbjdoyydnjrqrqejoyptzoklaxcjxbrrfmpdxckfjzahparhpanwqfjrpbslsyiwbldnpjqishlsuagevjmiyktgofvnyncizswldwnngnkifmaxbmospdeslxirofgqouaapfgltgqxdhurxljcepdpndqqgfwkfiqrwuwxfamciyweehktaegynfumwnhrgrhcluenpnoieqdivznrjljcotysnlylyswvdlkgsvrotavnkifwmnvgagjykxgwaimavqsxuitknmbxppgzfwtjdvegapcplreokicxcsbdrsyfpustpxxssnouifkypwqrywprjlyddrggkcglbgcrbihgpxxosmejchmzkydhquevpschkpyulqxgduqkqgwnsowxrmgqbmltrltzqmmpjilpfxocflpkwithsjlljxdygfvstvwqsyxlkknmgpppupgjvfgmxnwmvrfuwcrsadomyddazlonjyjdeswwznkaeaasyvurpgyvjsiltiykwquesfjmuswjlrphsdthmuqkrhynmqnfqdlwnwesdmiiqvcpingbcgcsvqmsmskesrajqwmgtdoktreqssutpudfykriqhblntfabspbeddpdkownehqszbmddizdgtqmobirwbopmoqzwydnpqnvkwadajbecmajilzkfwjnpfyamudpppuxhlcngkign"), String::from("rr")), 496);
    assert_eq!(maximum_subsequence_count(String::from("abdcdbc"), String::from("ac")), 4);
    assert_eq!(maximum_subsequence_count(String::from("aabb"), String::from("ab")), 6);
}

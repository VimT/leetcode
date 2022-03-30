//! 山羊拉丁文

pub fn to_goat_latin(sentence: String) -> String {
    sentence.split(' ').enumerate().map(|(i, x)| {
        let s = x.as_bytes();
        let mut result = s.to_vec();
        if !matches!(s[0], b'a'|b'e'|b'i'|b'o'|b'u'|b'A'|b'E'|b'I'|b'O'|b'U') {
            let first = result.remove(0);
            result.push(first);
        }
        result.push(b'm');
        result.push(b'a');
        for _ in 0..=i {
            result.push(b'a');
        }
        unsafe { String::from_utf8_unchecked(result) }
    }).collect::<Vec<_>>().join(" ")
}

fn main() {
    assert_eq!(to_goat_latin(String::from("I speak Goat Latin")), String::from("Imaa peaksmaaa oatGmaaaa atinLmaaaaa"));
    assert_eq!(to_goat_latin(String::from("The quick brown fox jumped over the lazy dog")), String::from("heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"));
}

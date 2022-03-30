//! 原子的数量

use std::collections::BTreeMap;

pub fn count_of_atoms(formula: String) -> String {
    let s = formula.as_bytes();
    let mut stack = vec![];
    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    let len = s.len();
    let mut i = 0;
    while i < len {
        if s[i] == b'(' {
            stack.push(map);
            map = BTreeMap::new();
            i += 1;
            continue;
        } else if s[i] == b')' {
            let mut ori_map = stack.pop().unwrap();
            let mut num_end = i + 1;
            let mut num = 0;
            while num_end < len && s[num_end].is_ascii_digit() {
                num = num * 10 + (s[num_end] - b'0') as i32;
                num_end += 1;
            }
            if num == 0 { num = 1; }
            for (k, v) in map {
                *ori_map.entry(k).or_default() += v * num;
            }
            map = ori_map;
            i = num_end;
            continue;
        }
        let mut atom_end = i + 1;
        while atom_end < len && s[atom_end].is_ascii_lowercase() { atom_end += 1; }
        let atom = unsafe { std::str::from_utf8_unchecked(&s[i..atom_end]) };
        let mut num_end = atom_end;
        while num_end < len && s[num_end].is_ascii_digit() { num_end += 1; }
        let mut num = 0;
        for j in atom_end..num_end {
            num = num * 10 + (s[j] - b'0') as i32;
        }
        if num == 0 { num = 1; }
        *map.entry(atom).or_default() += num;
        i = num_end;
    }

    let mut result = vec![];
    for (k, v) in map {
        result.extend_from_slice(k.as_bytes());
        if v > 1 {
            result.extend_from_slice(v.to_string().as_bytes());
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(count_of_atoms(String::from("H2O")), String::from("H2O"));
    assert_eq!(count_of_atoms(String::from("Mg(OH)2")), String::from("H2MgO2"));
    assert_eq!(count_of_atoms(String::from("K4(ON(SO3)2)2")), String::from("K4N2O14S4"));
}

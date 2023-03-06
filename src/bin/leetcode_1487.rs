//! 保证文件名唯一

use std::collections::HashMap;

pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut cnt: HashMap<String, i32> = HashMap::new();
    names.into_iter().map(|name| {
        let mut c = cnt.get(&name).cloned().unwrap_or(0);
        if c == 0 {
            cnt.insert(name.clone(), 1);
            return name;
        }
        while cnt.contains_key(&format!("{}({})", name, c)) {
            c += 1;
        }
        cnt.insert(name.clone(), c + 1);
        let result = format!("{}({})", name, c);
        cnt.insert(result.clone(), 1);
        result
    }).collect()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(names: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["pes","fifa","gta","pes(2019)"]), vec!["pes", "fifa", "gta", "pes(2019)"]);
        assert_eq!(func(svec!["gta","gta(1)","gta","avalon"]), vec!["gta", "gta(1)", "gta(2)", "avalon"]);
        assert_eq!(func(svec!["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece"]), vec!["onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece(4)"]);
        assert_eq!(func(svec!["wano","wano","wano","wano"]), vec!["wano","wano(1)","wano(2)","wano(3)"]);
        assert_eq!(func(svec!["kaido","kaido(1)","kaido","kaido(1)"]), vec!["kaido","kaido(1)","kaido(2)","kaido(1)(1)"]);
        assert_eq!(func(svec!["kaido","kaido(1)","kaido","kaido(1)","kaido(2)"]), vec!["kaido","kaido(1)","kaido(2)","kaido(1)(1)","kaido(2)(1)"]);
    }
    test(get_folder_names);
}

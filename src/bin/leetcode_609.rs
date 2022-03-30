//! 在系统中查找重复文件

use std::collections::HashMap;

use leetcode::svec;

pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    let mut content_map: HashMap<String, Vec<String>> = HashMap::new();
    for path in paths {
        let mut split = path.split(' ');
        let dir = split.next().unwrap();
        while let Some(file) = split.next() {
            let content_start = file.find('(').unwrap();
            let filename = &file[..content_start];
            let content = &file[content_start..];
            content_map.entry(content.to_string()).or_default().push(format!("{}/{}", dir, filename));
        }
    }
    content_map.into_iter().filter(|x| x.1.len() > 1).map(|x| x.1).collect()
}

fn main() {
    assert_eq!(find_duplicate(svec!["root/a 1.txt(abcd) 2.txt(efsfgh)", "root/c 3.txt(abdfcd)", "root/c/d 4.txt(efggdfh)"]).is_empty(), true);
    assert_eq!(find_duplicate(svec!["root/a 1.txt(abcd) 2.txt(efgh)", "root/c 3.txt(abcd)", "root/c/d 4.txt(efgh)", "root 4.txt(efgh)"]), vec![vec!["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"], vec!["root/a/1.txt", "root/c/3.txt"]]);
    assert_eq!(find_duplicate(svec!["root/a 1.txt(abcd) 2.txt(efgh)", "root/c 3.txt(abcd)", "root/c/d 4.txt(efgh)"]), vec![vec!["root/a/2.txt", "root/c/d/4.txt"], vec!["root/a/1.txt", "root/c/3.txt"]]);
}

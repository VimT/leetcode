//! 设计内存文件系统

use std::collections::HashMap;

struct File {
    content: String,
}

impl File {
    fn new() -> Self {
        File { content: String::new() }
    }
}

struct Dir {
    children: HashMap<String, Item>,
}

impl Dir {
    fn new() -> Self {
        Dir { children: HashMap::new() }
    }
}

enum Item {
    File(File),
    Dir(Dir),
}

impl Default for Item {
    fn default() -> Self {
        return Item::Dir(Dir::new());
    }
}

struct FileSystem {
    root: Dir,
}


impl FileSystem {
    fn new() -> Self {
        Self { root: Dir::new() }
    }

    fn ls(&self, path: String) -> Vec<String> {
        let (path, filename) = path.rsplit_once("/").unwrap();
        let mut cur_dir = &self.root;
        for dir in path.split("/").filter(|x| !x.is_empty()) {
            if let Some(sub) = cur_dir.children.get(dir) {
                if let Item::Dir(sub_dir) = sub {
                    cur_dir = sub_dir;
                } else {
                    return vec![];
                }
            } else {
                return vec![];
            }
        }
        if !filename.is_empty() {
            if let Some(item) = cur_dir.children.get(filename) {
                match item {
                    Item::File(_) => {
                        return vec![filename.to_string()];
                    }
                    Item::Dir(dir) => {
                        cur_dir = dir;
                    }
                }
            } else {
                return vec![];
            }
        }
        let mut result: Vec<String> = cur_dir.children.keys().map(|x| x.to_string()).collect();
        result.sort_unstable();
        result
    }

    fn mkdir(&mut self, path: String) {
        let mut cur_dir = &mut self.root;
        for dir in path.split("/").filter(|x| !x.is_empty()) {
            let sub = cur_dir.children.entry(dir.to_string()).or_default();
            if let Item::Dir(sub_dir) = sub {
                cur_dir = sub_dir;
            } else {
                unreachable!();
            }
        }
    }

    fn add_content_to_file(&mut self, file_path: String, content: String) {
        let (path, filename) = file_path.rsplit_once("/").unwrap();
        let mut cur_dir = &mut self.root;
        for dir in path.split("/").filter(|x| !x.is_empty()) {
            let sub = cur_dir.children.entry(dir.to_string()).or_default();
            if let Item::Dir(sub_dir) = sub {
                cur_dir = sub_dir;
            } else { unreachable!() }
        }
        let item = cur_dir.children.entry(filename.to_string()).or_insert_with(|| Item::File(File::new()));
        if let Item::File(file) = item {
            file.content += &content;
        }
    }

    fn read_content_from_file(&self, file_path: String) -> String {
        let (path, filename) = file_path.rsplit_once("/").unwrap();
        let mut cur_dir = &self.root;
        for dir in path.split("/").filter(|x| !x.is_empty()) {
            if let Some(sub) = cur_dir.children.get(dir) {
                if let Item::Dir(sub_dir) = sub {
                    cur_dir = sub_dir;
                }
            }
        }
        if let Item::File(file) = cur_dir.children.get(filename).unwrap() {
            return file.content.to_string();
        }
        String::new()
    }
}

fn main() {
    let mut fs = FileSystem::new();
    assert_eq!(fs.ls(String::from("/")), Vec::<String>::new());
    fs.mkdir(String::from("/a/b/c"));
    fs.add_content_to_file(String::from("/a/b/c/d"), String::from("hello"));
    assert_eq!(fs.ls(String::from("/")), ["a"]);
    assert_eq!(fs.ls(String::from("/a/b/c/d")), ["d"]);
    assert_eq!(fs.read_content_from_file(String::from("/a/b/c/d")), "hello");
}

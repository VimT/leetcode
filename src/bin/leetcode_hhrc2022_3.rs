//! 重复的彩灯树

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

/// 找相同子树
pub fn light_distribution(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn walk(
        root: Option<Rc<RefCell<TreeNode>>>,
        ids: &mut HashMap<(i32, i32, i32), i32>,
        ret: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> i32 {
        if root.is_none() {return 0;}
        let node = root.as_ref().unwrap().borrow();
        let lid = walk(node.left.clone(), ids, ret);
        let rid = walk(node.right.clone(), ids, ret);
        if let Some(id) = ids.get_mut(&(lid, node.val, rid)) {
            if *id > 0 {
                ret.push(root.clone());
                *id = -*id;
            }
            -*id
        } else {
            let id = ids.len() as i32 + 1;
            ids.insert((lid, node.val, rid), id);
            id
        }
    }


    let mut ids = HashMap::new();
    let mut ret = Vec::new();
    walk(root, &mut ids, &mut ret);
    ret
}

fn main() {
    assert_eq!(light_distribution(tree![1,3,3,null,2,null,2]), vec![tree![2], tree![3,null,2]])
}

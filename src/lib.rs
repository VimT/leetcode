pub mod algorithm;
pub mod avl;
pub mod fnv;
pub mod gcd;
pub mod linknode;
pub mod treenode;

pub fn unorder<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort_unstable();
    list
}

pub fn unorder_deep<T: Ord>(mut list: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for item in &mut list {
        item.sort_unstable();
    }
    list.sort_unstable();
    list
}

#[macro_export]
macro_rules! svec {
    () => (
        Vec::<String>::new()
    );
    ($($x:expr),+ $(,)?) => (
        vec![$(String::from($x)),+]
    );
}

#[macro_export]
macro_rules! link {
    () => (
        Option::<Box<ListNode>>::None
    );
    ($($x:expr),+ $(,)?) => (
        vec_to_link(vec![$($x),+])
    );
}

#[macro_export]
macro_rules! tree {
    () => (
        Option::<Rc<RefCell<TreeNode>>>::None
    );
    ($($x:expr),+$(,)?) => (
        leetcode_tree(stringify!($($x),+))
    );
}

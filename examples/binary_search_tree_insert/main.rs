use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

type ORTree = Option<Rc<RefCell<TreeNode>>>;

fn create_tree_node(v: &Vec<i32>) -> ORTree {
    if v.is_empty() {
        return None
    }
    let root = Rc::new(RefCell::new(TreeNode::new(v[0])));
    let mut node = vec![root.clone()];
    let mut i = 1usize;
    let len = v.len();
    while !node.is_empty() {
        if i >= len {
            break;
        }
        if v[i] != -1 {
            let left = Rc::new(RefCell::new(TreeNode::new(v[i])));
            node[0].borrow_mut().left = Some(left.clone());
            node.push(left);
        } else {
            node[0].borrow_mut().left = None;
        }
        i += 1;
        if i >= len {
            break;
        }
        if v[i] != -1 {
            let right = Rc::new(RefCell::new(TreeNode::new(v[i])));
            node[0].borrow_mut().right = Some(right.clone());
            node.push(right);
        } else {
            node[0].borrow_mut().right = None;
        }
        i += 1;
        node.remove(0);
    }
    Some(root)
}

fn main() {
    let t1 = create_tree_node(&vec![]);
    // println!("{:#?}", t1);
    println!("{:#?}", insert_into_bst(t1, 5));
}

fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn f(root: ORTree, val: i32) {
        let root = root.unwrap();
        let mut root = root.borrow_mut();
        if root.val > val {
            if root.left.is_some() {
                f(root.left.clone(), val)
            } else {
                root.left = create_tree_node(&vec![val])
            }
        } else {
            if root.right.is_some() {
                f(root.right.clone(), val)
            } else {
                root.right = create_tree_node(&vec![val])
            }
        }
    }
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
    f(root.clone(), val);
    root
}

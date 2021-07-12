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
    let t1 = create_tree_node(&vec![1,2,3,4,5,-1,7]);
    println!("{:#?}", connect(t1));
}

fn connect(root: ORTree) -> ORTree {
    let mut v = vec![root.clone()];
    while !v.is_empty() {
        let mut last: ORTree = None;
        let len = v.len();
        for i in 1..len + 1 {
            let r = v.remove(0);
            if let Some(r) = r {
                if r.borrow().left.is_some() {
                    v.push(r.borrow().left.clone());
                }
                if r.borrow().right.is_some() {
                    v.push(r.borrow().right.clone());
                }
                if i != 1 {
                    last.unwrap().borrow_mut().next = Some(r.clone());
                }
                last = Some(r.clone());
            }
        }
    }
    root
}

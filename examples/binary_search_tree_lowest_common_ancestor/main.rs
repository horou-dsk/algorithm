use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
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
    let instant = Instant::now();
    let t1 = create_tree_node(&vec![6,2,8,0,4,7,9,-1,-1,3,5]);
    println!("{:?}", lowest_common_ancestor(t1, create_tree_node(&vec![2]), create_tree_node(&vec![8])).unwrap().borrow().val);
    println!("{:?}", instant.elapsed());
}

fn lowest_common_ancestor(root: ORTree, p: ORTree, q: ORTree) -> ORTree {
    match (root.clone(), p.clone(), q.clone()) {
        (Some(r), Some(sp), Some(sq)) => {
            let (mut r, pv, qv) = (r.borrow_mut(), sp.borrow().val, sq.borrow().val);
            if r.val > pv.max(qv) {
                return lowest_common_ancestor(r.left.take(), p, q)
            }
            if r.val < pv.min(qv) {
                return lowest_common_ancestor(r.right.take(), p, q)
            }
            root
        },
        _ => None
    }
    // let (r, pv, qv) = (root.unwrap(), p.unwrap(), q.unwrap());
    // let (mut r, pv, qv) = (r.borrow_mut(), pv.borrow().val, qv.borrow().val);
    // if r.val > pv.max(qv) {
    //     return lowest_common_ancestor(r.left.take(), p, q)
    // }
    // if r.val < pv.min(qv) {
    //     return lowest_common_ancestor(r.right.take(), p, q)
    // }
    // root
}

use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

// Definition for a binary tree node.
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

fn create_tree_node(v: &Vec<i32>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if index >= v.len() || v[index] == 0 {
        return None
    }
    let node = Rc::new(RefCell::new(TreeNode::new(v[index])));
    node.borrow_mut().left = create_tree_node(v, index * 2 + 1);
    node.borrow_mut().right = create_tree_node(v, index * 2 + 2);
    Some(node)
}

fn main() {
    let instant = Instant::now();
    let t1 = create_tree_node(&vec![1,3,2,5], 0);// Some(Rc::new(RefCell::new(TreeNode::new())));
    let t2 = create_tree_node(&vec![2,1,3,0,4,0,7], 0);
    // println!("{:#?}", t1);
    println!("{:#?}", merge_trees(t1, t2));
    println!("{:?}", instant.elapsed());
}

type RTNode = Option<Rc<RefCell<TreeNode>>>;

fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn merge(t1: RTNode, t2: RTNode) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(a), Some(b)) => {
                {
                    let (mut v1, mut v2) = (a.borrow_mut(), b.borrow_mut());
                    v1.val += v2.val;
                    let left = v1.left.take();
                    v1.left = merge(left, v2.left.take());
                    let right = v1.right.take();
                    v1.right = merge(right, v2.right.take());
                }
                Some(a)
            },
            (None, b) => b,
            (a, None) => a,
        }
    }
    merge(t1, t2)
}

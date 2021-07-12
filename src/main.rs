use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Box<Option<TreeNode>>,
    pub right: Box<Option<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: Box::new(None),
            right: Box::new(None)
        }
    }
}

fn bbc(a: &mut TreeNode) {
    a.val = 5;
}

fn main() {
    let instant = Instant::now();
    let t = TreeNode {
        val: 1,
        left: Box::from(Some(TreeNode::new(2))),
        right: Box::from(Some(TreeNode::new(3))),
    };
    let mut b = t.left.unwrap();
    bbc(&mut b);
    println!("{}", b.val);
    println!("{:?}", instant.elapsed());
}

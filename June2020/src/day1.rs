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

use std::collections::VecDeque;
// Breadth-first traversal?
fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

    match root {
        Some(ref node) => {
            q.push_back(node.clone());
        },
        None => return
    }
    
    let mut layer_len = 1;
    let mut layer_i = 0;
    loop {
        match q.pop_front() {
            Some(node) => {
                layer_i += 1;
                if layer_i == layer_len {
                    println!("{}", node.borrow().val);
                    layer_len *= 2;
                    layer_i = 0;
                } else {
                    print!("{} ", node.borrow().val);
                }

                if let Some(l) = node.borrow().left.as_ref() {
                    q.push_back(l.clone());
                }
                if let Some(r) = node.borrow().right.as_ref() {
                    q.push_back(r.clone());
                }
            }
            None => break,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(ref node) => {
                let inv_left = Solution::invert_tree(node.borrow().left.clone());
                let inv_right = Solution::invert_tree(node.borrow().right.clone());

                node.borrow_mut().left = inv_right;
                node.borrow_mut().right = inv_left;
            },
            None => (),
        }
        root
    }
}

pub fn run() {
    println!("Day 1: Binary Tree Inversion");
    let mut root = TreeNode::new(4);
    let mut left = TreeNode::new(2);
    left.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    left.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left = Some(Rc::new(RefCell::new(left)));
    let mut right = TreeNode::new(7);
    right.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    right.right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.right = Some(Rc::new(RefCell::new(right)));

    println!("Original tree:");
    let tree = Some(Rc::new(RefCell::new(root)));
    print_tree(&tree);

    println!("Inverted tree:");
    let inv_tree = Solution::invert_tree(tree);
    print_tree(&inv_tree);
}
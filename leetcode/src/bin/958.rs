#![allow(dead_code)]

// Definition for a binary tree node.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut end = false;
        while let Some(node_opt) = queue.pop_front() {
            if let Some(node_rc) = node_opt {
                if end {
                    return false;
                }
                queue.push_back(node_rc.borrow().left.clone());
                queue.push_back(node_rc.borrow().right.clone());
            } else {
                end = true;
            }
        }
        true
    }
}

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
            right: None,
        }
    }
}

struct Solution;

fn main() {
    // [1,2,3,4,5,6]
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    root.borrow_mut().left = Some(Rc::clone(&node2));
    root.borrow_mut().right = Some(Rc::clone(&node3));
    node2.borrow_mut().left = Some(Rc::clone(&node4));
    node2.borrow_mut().right = Some(Rc::clone(&node5));
    node3.borrow_mut().left = Some(Rc::clone(&node6));
    assert_eq!(Solution::is_complete_tree(Some(root)), true);

    // [1,2,3,4,5,null,6]
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    root.borrow_mut().left = Some(Rc::clone(&node2));
    root.borrow_mut().right = Some(Rc::clone(&node3));
    node2.borrow_mut().left = Some(Rc::clone(&node4));
    node2.borrow_mut().right = Some(Rc::clone(&node5));
    node3.borrow_mut().right = Some(Rc::clone(&node6));
    assert_eq!(Solution::is_complete_tree(Some(root)), false);
}

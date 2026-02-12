use super::Solution;
use std::rc::Rc;
use std::cell::RefCell;

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

impl Solution {
    pub fn level_median(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
        let mut vals: Vec<i32> = vec![];
        Self::preorder_traversal(root, level, 0, &mut vals);
        
        vals.get(vals.len() / 2).copied().unwrap_or(-1)
    }

    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, target_level :i32, cur_level: i32, vals: &mut Vec<i32>) {
        match root {
          Some(root_node) => {
            if cur_level == target_level {
              vals.push(root_node.borrow().val);
              return;
            }

            if cur_level < target_level {
              Self::preorder_traversal(root_node.borrow().left.clone(), target_level, cur_level, vals);
              Self::preorder_traversal(root_node.borrow().right.clone(), target_level, cur_level, vals);
            }
          },
          None => { return; }
        } 
    }
}

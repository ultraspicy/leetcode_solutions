use super::Solution;

use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;

//Definition for a binary tree node.
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
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {

        let mut m: BTreeMap<i32, BTreeMap<i32, Vec<i32>>> = BTreeMap::new();

        if root == None {
            return vec![];
        }

        let mut q= VecDeque::new();
        q.push_back((0, 0, root));

        while !q.is_empty() {
            if let Some((row, col, h_opt)) = q.pop_front() {
                if let Some(node_rc) = h_opt {
                    let node = node_rc.borrow();
                    m.entry(col).or_insert(BTreeMap::new()).entry(row).or_insert(Vec::new()).push(node.val);
                    q.push_back((row + 1, col - 1, node.left.clone()));
                    q.push_back((row + 1, col + 1, node.right.clone()));
                }
            }
        }

        let mut ret = vec![];
        m.values_mut().for_each(|col_map| {
            let mut col = vec![];
            for cell in col_map.values_mut() {
                cell.sort();
                col.append(cell);
            }
            ret.push(col);
        });

        ret
    }
}

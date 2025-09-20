use super::Solution;

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
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root == None {
            return vec![]
        }

        let mut column_to_nums: HashMap<i32, Vec<i32>> = HashMap::new();
        let (mut l, mut r) = (0, 0);
        let mut ret = vec![];

        let mut q: VecDeque<(i32, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        q.push_back((0, root.unwrap()));
        
        while !q.is_empty() {
            if let Some((idx, node)) = q.pop_front() {
                column_to_nums.entry(idx).or_insert_with(Vec::new).push(node.borrow().val);
                l = l.min(idx);
                r = r.max(idx);

                let node_ref = node.borrow();
                if let Some(l) = node_ref.left.clone() {
                    q.push_back((idx - 1, l));
                }
                if let Some(r) = node_ref.right.clone() {
                    q.push_back((idx + 1, r));
                }
            }
        }

        for i in l..=r {
            ret.push(column_to_nums.get(&i).cloned().unwrap());
        }

        ret
    }
}
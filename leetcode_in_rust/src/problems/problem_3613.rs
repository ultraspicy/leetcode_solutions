use std::{cmp::Reverse, collections::BinaryHeap};

use super::Solution;

#[derive(Debug, Default)]
struct DSU {
    num_of_partitiom: usize,
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            num_of_partitiom: n,
            root: (0..n).collect::<Vec<_>>(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if n != self.root[n] {
            self.root[n] = self.find(self.root[n]);
        }
        self.root[n]
    }

    fn union(&mut self, m: usize, n: usize) {
        let m_root = self.find(m);
        let n_root = self.find(n);

        if m_root == n_root {
            return;
        }
        self.num_of_partitiom -= 1;
        if self.rank[m_root] < self.rank[n_root] {
            self.root[m_root] = n_root;
        } else if self.rank[m_root] > self.rank[n_root] {
            self.root[n_root] = m_root;
        } else {
            self.root[m_root] = n_root;
            self.rank[n_root] += 1;
        }
    }
}

impl Solution {
    pub fn min_cost_3613(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dsu = DSU::new(n);

        let mut heap = BinaryHeap::new();
        edges.iter()
            .for_each(|edge| {
                heap.push(Reverse((edge[2], edge[0], edge[1])));
            });

        while !heap.is_empty() {
            if let Some(Reverse((weight, u, v))) = heap.pop() {
                dsu.union(u as usize, v as usize);
                if dsu.num_of_partitiom == k {
                    return weight;
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_cost_3613(5, vec![vec![0,1,4], vec![1,2,3], vec![1,3,2], vec![3,4,6]], 2);
        assert_eq!(ret, 4)
   }
}

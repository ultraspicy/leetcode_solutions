use std::{collections::{HashMap, HashSet}, hash::Hash, i32, vec};

use super::Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n: usize = nums.len();
        let mut graph = vec![vec![false; n]; n];
        edges.iter().for_each(|edge| {
            let (n1, n2) = (edge[0] as usize, edge[1] as usize);
            graph[n1][n2] = true;
            graph[n2][n1] = true;
        });

        let mut descendants: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut xor_all = vec![0; n];

        // dfs from node_id = 0
        let mut visited = vec![false; n];
        visited[0] = true;
        Self::dfs_2322(n, 0, &graph, &nums, &mut descendants, &mut xor_all, &mut visited);

        println!("descendants {:?}", descendants);
        println!("xor_all {:?}", xor_all);
        println!("visited {:?}", visited);

        let mut ret = i32::MAX;
        for i in 0..edges.len() {
            let (s1, e1) = (edges[i][0] as usize, edges[i][1] as usize);
            let node1 = if descendants[s1].contains(&e1) {e1} else {s1};
            for j in (i + 1)..edges.len() {
                let (s2, e2) = (edges[j][0] as usize, edges[j][1] as usize);
                let node2 = if descendants[s2].contains(&e2) {e2} else {s2};

                let (mut left, mut mid, mut right) = (0, 0, 0);
                if descendants[node1].contains(&node2) {
                    (left, mid, right) = (xor_all[node2], xor_all[node1] ^ xor_all[node2], xor_all[0] ^ xor_all[node1]);
                } else if descendants[node2].contains(&node1) {
                    (left, mid, right) = (xor_all[node1], xor_all[node2] ^ xor_all[node1], xor_all[0] ^ xor_all[node2]);
                } else {
                    (left, mid, right) = (xor_all[node1], xor_all[node2], xor_all[0] ^ xor_all[node2] ^ xor_all[node1]);
                }

                ret = ret.min(left.max(mid).max(right) - left.min(mid).min(right));
                println!("node1 = {}, node2 = {}", node1, node2);
                println!("left, mid, right {:?}", (left, mid, right));
                println!("i, j, ret {}, {}, {:?}", i, j, ret);
            }
        }
        ret
    }

    fn dfs_2322(
        n: usize,
        current: usize,
        graph: &Vec<Vec<bool>>,
        nums: &Vec<i32>,
        descendants: &mut Vec<HashSet<usize>>,
        xor_all: &mut Vec<i32>,
        visited: &mut Vec<bool>
    ) {
        for (descendant_id, &connected) in graph[current].iter().enumerate() {
            if connected && !visited[descendant_id] {
                visited[descendant_id] = true;
                // dfs
                Self::dfs_2322(n, descendant_id, graph, nums, descendants, xor_all, visited);
                // update self
                descendants[current].insert(descendant_id);
                let (left, right) = if current < descendant_id {
                    let (l, r) = descendants.split_at_mut(descendant_id);
                    (&mut l[current], &mut r[0])
                } else {
                    let (l, r) = descendants.split_at_mut(current);
                    (&mut r[0], &mut l[descendant_id])
                };
                left.extend(right.iter());
                xor_all[current] = xor_all[current] ^ xor_all[descendant_id];
            }
        }
        xor_all[current] = xor_all[current] ^ nums[current];
    }
 }

 #[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::minimum_score(vec![1,5,5,4,11], vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4]]);
        assert_eq!(ret, 9)
    }
 }

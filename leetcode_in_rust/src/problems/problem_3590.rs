use std::{collections::{BTreeSet, HashMap}};

use super::Solution;

impl Solution {
    pub fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::new();

        // build tree
        for i in 1..par.len() {
            graph.entry(par[i]).or_insert_with(Vec::new).push(i as i32);
        }
        let mut path_sums = HashMap::new();
        Self::dfs_to_populate_path_sum(&graph, &vals, &mut path_sums, 0, vals[0]);

        // build result, also build a cache alongside
        let mut ret = vec![];
        let mut cache: HashMap<i32, BTreeSet<i32>> = HashMap::new();

        queries.iter().for_each(|query| {
            let (node_id, kth) = (query[0], query[1]);
            if cache.get(&node_id).is_none() {
                Self::construct_ordered_set_for_node(&graph, &path_sums, node_id, &mut cache);
            }
            let default_query_result = -1;
            ret.push(*cache.get(&node_id).unwrap().iter().nth(kth as usize - 1).unwrap_or(&default_query_result));
        });

        ret
    }

    fn construct_ordered_set_for_node(graph: &HashMap<i32, Vec<i32>>, path_sums: &HashMap<i32, i32>, node_id: i32, cache: &mut HashMap<i32, BTreeSet<i32>>) {
        if graph.get(&node_id).is_none() {
            // we already at leaf
            let mut ordered_set = BTreeSet::new();
            ordered_set.insert(path_sums.get(&node_id).copied().unwrap());
            cache.insert(node_id, ordered_set);
            return;
        }

        let mut ordered_set = BTreeSet::new();
        for &child in graph.get(&node_id).unwrap().iter() {
            Self::construct_ordered_set_for_node(graph, path_sums, child, cache);
            ordered_set.extend(cache.get(&child).unwrap());
        }
        ordered_set.insert(path_sums.get(&node_id).copied().unwrap());
        cache.insert(node_id, ordered_set);
    }

    fn dfs_to_populate_path_sum(graph: &HashMap<i32, Vec<i32>>, vals: &Vec<i32>, path_sums:&mut HashMap<i32, i32>, current_node: i32, current_xor: i32) {
        path_sums.insert(current_node, current_xor);

        if let Some(children) = graph.get(&current_node) {
            children.iter().for_each(|&child| {
                Self::dfs_to_populate_path_sum(graph, vals, path_sums, child, current_xor ^ vals[child as usize]);
            });

        }
    }

}
// extend() Source mutability can be immutable reference, source after operation unchanged, slower
// append() Must be &mut, Always emptied
// ordered_set -> BTreeSet, and nth_back()
// Option -> is_none()
// into() from tuple to map
// copied() on enum, unwrap_or(-1)

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::kth_smallest(vec![-1,0,0], vec![1,1,1], vec![vec![0,1],vec![0,2],vec![0,3]]);
        assert_eq!(vec![0,1,-1], ret);
    }

    #[test]
    fn unit_test_example2() {
        let ret = Solution::kth_smallest(vec![-1,0,1], vec![5,2,7], vec![vec![0,1],vec![1,2],vec![1,3],vec![2,1]]);
        assert_eq!(vec![0,7,-1,0], ret);
    }
}

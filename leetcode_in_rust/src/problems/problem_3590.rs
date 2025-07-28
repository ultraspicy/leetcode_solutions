use std::{collections::{BTreeSet, HashMap, HashSet}, hash::Hash};

use super::Solution;

impl Solution {
    pub fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::new();

        // build tree
        for i in 1..par.len() {
            graph.entry(par[i]).or_insert_with(Vec::new).push(i as i32);
        }

        // let mut path_sums = HashMap::new();
        // let mut children_list = HashMap::new();
        // Self::dfs_3590(0, &graph, &vals, &mut path_sums, &mut children_list);

        // let mut ret = vec![];
        // let mut cache: HashMap<i32, BTreeSet<i32>> = HashMap::new();

        // queries.iter().for_each(|query| {
        //     let (node_id, kth) = (query[0], query[1]);
        // });
        // ret

        vec![]
    }

    // fn dfs_3590(current: i32, graph: &HashMap<i32, Vec<i32>>, vals: &Vec<i32>, path_sums:&mut HashMap<i32, i32>, children_list: &mut HashMap<i32, Vec<i32>>) {
    //     if graph.get(&current). is_none() {
    //         path_sums.insert(current, vals[current as usize]);
    //         return;
    //     }

    //     let mut self_val = vals[current as usize];
    //     let mut children_vec = vec![];

    //     if let Some(children) = graph.get(&current) {
    //         for next_child in children {
    //             self_val = self_val ^ Self::dfs_3590(*next_child, graph, vals, path_sums, children_list);
    //             children_vec.extend(children_list.get(next_child));
    //         }
    //         path_sums.insert(current, self_val);
    //         children_list.insert(current, children_vec);
    //     }
    // }
}
// extend() Source mutability can be immutable reference, source after operation unchanged, slower
// append() Must be &mut, Always emptied
// BTreeSet, and nth_back()
// is_none()
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
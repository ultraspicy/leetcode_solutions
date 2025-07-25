use std::collections::{BinaryHeap, HashMap};

use super::Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let k = k as usize;
        let end_node_id = online.len() - 1;
        let mut ret = -1;
        let mut graph = HashMap::new();
        let mut scores = HashMap::new();

        edges.iter().for_each(|edge|{
            let (start_node, end_node, score) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);
            if online[start_node] && online[end_node] {
                graph.entry(start_node).or_insert_with(Vec::new).push(end_node);
                scores.insert((start_node, end_node), score);
                if end_node == end_node_id && start_node == 0 && score <= k {
                    ret = ret.max(score as i32);
                }
            }
        });

        // println!("graph = {:?}", graph);
        // println!("scores = {:?}", scores);

        // tuple (current_score, accumulated_sum, end_node_id)
        let mut heap = BinaryHeap::new();
        if let Some(neighbours) = graph.get(&0) {
            for &neighbour in neighbours {
                let current_score = *scores.get(&(0, neighbour)).unwrap();
                heap.push((current_score, current_score, neighbour));
            }
        } else {
            return -1;
        }

        while let Some((current_score, accumulated_sum, node_id)) = heap.pop() {
            if let Some(neighbours) = graph.get(&node_id) {
                for &neighbour in neighbours {
                    let new_weight = *scores.get(&(node_id, neighbour)).unwrap();
                    //println!("node_id {:?}, neighbour = {}, new_weight = {}", node_id, neighbour, new_weight);
                    if accumulated_sum + new_weight <= k {
                        if neighbour == end_node_id {
                            ret = ret.max(current_score.min(new_weight) as i32);
                        }
                        heap.push((current_score.min(new_weight), accumulated_sum + new_weight, neighbour));
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::find_max_path_score(vec![vec![0,1,5],vec![1,3,10],vec![0,2,3],vec![2,3,4]],
            vec![true, true, true, true], 10);
        assert_eq!(ret, 3);
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::find_max_path_score(
            vec![vec![0,1,7],vec![1,4,5],vec![0,2,6],vec![2,3,6],vec![3,4,2],vec![2,4,6]],
            vec![true,true,true,false,true],
            12
        );
        assert_eq!(ret, 6);
    }
}

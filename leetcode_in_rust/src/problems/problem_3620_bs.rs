use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};
use super::Solution;

// use binary search so we can avoid revisiting (exhasting all possible values in original graph)
impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let (mut start, mut end) = (0, 1000_000_001);

        while start + 1 < end {
            let mid = (start + end) / 2;
            if Self::find_path_score_at_least(&edges, &online, k, mid) {
                start = mid;
            } else {
                end = mid;
            }
            println!("start = {}, end ={}", start, end)
        }
        if Self::find_path_score_at_least(&edges, &online, k, end) {
            end
        } else {
            if Self::find_path_score_at_least(&edges, &online, k, start) {
                start
            } else {
                -1
            }
        }
    }

    pub fn find_path_score_at_least(edges: &Vec<Vec<i32>>, online: &Vec<bool>, k: i64, threshold: i32) -> bool {
        let mut graph = HashMap::new();
        let mut scores = HashMap::new();
        let final_node_id = (online.len() - 1) as i32;

        edges.iter().for_each(|edge| {
            let (start_node, end_node, score) = (edge[0], edge[1], edge[2]);
            if score >= threshold && (score as i64 <= k) && online[start_node as usize] && online[end_node as usize]  {
                graph.entry(start_node).or_insert_with(Vec::new).push(end_node);
                scores.insert((start_node, end_node), score);
            }
        });
        println!("graph = {:?}", graph);
        println!("scores = {:?}", scores);

        // tuple (accumulated_sum, current_score, end_node_id)
        let mut visited = vec![false; online.len()];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));

        while let Some(Reverse((accumulated_sum, node_id))) = heap.pop() {
            println!("node_id = {}", node_id);

            if node_id == final_node_id {
                return true;
            }
            if visited[node_id as usize] {
                continue;
            }

            if let Some(neighbour) = graph.get(&node_id) {
                for &neigh in neighbour {
                    let new_score = *scores.get(&(node_id, neigh)).unwrap() as i64;
                    println!("node_id, neigh, new_score = {}, {}, {}", node_id, neigh, new_score);
                    if accumulated_sum + new_score <= k {
                        heap.push(Reverse((accumulated_sum + new_score, neigh)));
                    }
                }
            }
            visited[node_id as usize] = true;
        }
        
        false
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn helper_test() {
        let ret = Solution::find_path_score_at_least(&vec![vec![0,1,5],vec![1,3,10],vec![0,2,3],vec![2,3,4]],
            &vec![true, true, true, true], 10, 1);
        assert_eq!(ret, true);
    }

    #[test]
    fn unit_test() {
        let ret = Solution::find_max_path_score(vec![vec![0,1,5],vec![1,3,10],vec![0,2,3],vec![2,3,4]],
            vec![true, true, true, true], 10);
        assert_eq!(ret, 3);
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::find_max_path_score_normal(
            vec![vec![0,1,7],vec![1,4,5],vec![0,2,6],vec![2,3,6],vec![3,4,2],vec![2,4,6]],
            vec![true,true,true,false,true],
            12
        );
        assert_eq!(ret, 6);
    }
}
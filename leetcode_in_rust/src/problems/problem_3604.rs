use std::{any, collections::{BinaryHeap, HashMap, VecDeque}};

use super::Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
struct NodeMinTime {
    node: i32,
    min_time: i32,
}

impl Ord for NodeMinTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_time.cmp(&self.min_time) 
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for NodeMinTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_time_3604(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<Vec<i32>>>= HashMap::new();
        
        // build map
        edges.iter().for_each(|e|{
            graph.entry(e[0]).or_insert_with(Vec::new).push(vec![e[1], e[2], e[3]]);
        });

        // bfs via heap 
        let mut heap: BinaryHeap<NodeMinTime> = BinaryHeap::new();
        heap.push(NodeMinTime { node: 0, min_time: 0 });
        let mut visited = vec![false; n as usize];

        while !heap.is_empty() {
            let head = heap.pop().unwrap();
            if head.node == n - 1 {
                return head.min_time;
            }
            
            if !visited[head.node as usize] {
                visited[head.node as usize] = true;
            } else {
                continue;
            }
            if let Some(outgoing_edges) = graph.get(&head.node) {
                outgoing_edges.iter()
                    .for_each(|out_edge| {
                        let current = head.min_time;
                        if current <= out_edge[2] {
                            heap.push(NodeMinTime { node: out_edge[0], min_time: current.max(out_edge[1]) + 1});
                        }
                    });
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_1() {
        let rst = Solution::min_time_3604(3, vec![vec![0,1,0,1], vec![1,2,2,5]]);
        assert_eq!(rst, 3);
    }

    #[test]
    fn unit_test_2() {
        let rst = Solution::min_time_3604(4, vec![vec![0,1,0,3], vec![1,3,7,8], vec![0,2,1,5], vec![2,3,4,7]]);
        assert_eq!(rst, 5);
    }

    #[test]
    fn unit_test_3() {
        let rst = Solution::min_time_3604(3, vec![vec![1,0,1,3], vec![1,2,3,5]]);
        assert_eq!(rst, -1);
    }

    #[test]
    fn unit_test_4() {
        let rst = Solution::min_time_3604(9, vec![
            vec![1,4,3,23], vec![3,5,34,66], vec![6,8,73,77], vec![0,3,17,63],
            vec![7,6,43,63], vec![2,4,64,67], vec![4,2,10,56], vec![5,8,28,35],
            vec![0,3,56,70], vec![5,3,55,55], vec![8,7,17,29], vec![8,1,10,46],
            vec![1,6,0,37], vec![3,5,33,60], vec![3,1,31,77], vec![0,1,17,22],
            vec![4,8,26,43], vec![8,4,67,72], vec![3,0,13,16], vec![4,6,52,52],
            vec![7,1,9,44], vec![3,4,18,65], vec![2,6,64,79], vec![8,2,75,81],
            vec![7,2,15,68]
        ]);
        assert_eq!(rst, 27);
    }
}
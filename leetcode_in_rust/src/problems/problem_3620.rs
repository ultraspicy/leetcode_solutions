use super::Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        
    }

    // fn dfs_3620(start_node: i32, end_node: i32, connections: &Vec<Vec<bool>>, current: &Vec<cost_and_score>) -> Vec<cost_and_score> {
    //     // we are doing something similar with permutation, using a clone() is necessary instead of return an option and own the current
    //     if start_node == end_node {
    //         return current.clone();
    //     }

    //     let m = connections.len();
    //     let n = connections[0].len();
    //     connections[start_node as usize].iter()
    //         .enumerate()
    //         .filter(|&(_index, &connected)| {
    //             connected
    //         })
    //         .map(|(node_id, _)| {
                
    //         })
    //     vec![]
    // }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct cost_and_score {
    cost: i32,
    score: i32,
}   

impl PartialOrd for cost_and_score {
    fn partial_cmp(&self, other: &cost_and_score) -> Option<std::cmp::Ordering> { 
        return Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for cost_and_score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

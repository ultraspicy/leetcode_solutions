use super::Solution;

impl Solution {
    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let n = n as usize;

        // Use adjacency list instead of adjacency matrix
        let mut graph = vec![Vec::new(); n];
        edges.iter().for_each(|edge| {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        });

        let mut visited = vec![false; n];
        visited[0] = true;

        let (_total, num_of_mutation) = Self::dfs(n, 0, &graph, &mut visited, &cost);
        num_of_mutation as i32
    }

    fn dfs(n: usize, node_id: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, cost: &Vec<i32>) -> (usize, usize) {
        let mut is_leaf = true;
        let mut children_cost = Vec::new();

        // Iterate through connected nodes directly
        for &connected_node_id in &graph[node_id] {
            if !visited[connected_node_id] {
                is_leaf = false;
                visited[connected_node_id] = true;
                let (total, mutation) = Self::dfs(n, connected_node_id, graph, visited, cost);
                children_cost.push((total, mutation));
            }
        }

        if is_leaf {
            return (cost[node_id] as usize, 0);
        }

        let max_cost = children_cost.iter()
            .map(|&(total, _)| total)
            .max().unwrap();

        let mutation = children_cost.iter()
            .map(|&(total, mutation)| {
                if total == max_cost {
                    mutation
                } else {
                    mutation + 1
                }
            }).sum::<usize>();

        (max_cost + cost[node_id] as usize, mutation)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_increase(5, vec![vec![0,4], vec![0,1], vec![1,2], vec![1,3]], vec![3,4,1,1,7]);
        assert_eq!(ret, 1)
   }

   #[test]
    fn unit_test2() {
        let ret = Solution::min_increase(3, vec![vec![0,1], vec![0,2]], vec![2,1,3]);
        assert_eq!(ret, 1)
   }
}
